//! Recursive-descent parser for the `ValueDef::Expr` sugar.
//!
//! Grammar:
//! ```text
//! value   := sum
//! sum     := product (('+' | '-') product)*
//! product := unary (('*' | '/') unary)*
//! unary   := '-' unary | primary
//! primary := NUMBER | IDENT '(' args ')' | '(' value ')'
//! args    := role (',' value)? | value (',' value)?
//! role    := 'caster' | 'target'
//! IDENT   := attack | health | max_health | speed | min | max | random
//! ```
//!
//! `attack`/`health`/`max_health`/`speed` take exactly one role argument and
//! produce a `CreatureStat` value; `min`/`max`/`random` take exactly two
//! value arguments. Unary minus desugars to `Sub(0, x)`, which under the
//! runtime's saturating u16 arithmetic clamps negative results to 0.

use super::{
    selector::{CardinalityDef, SelectionDef, SelectorDef, SelectorKindDef},
    value::ValueDef,
};
use crate::actions::value_source::StatType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueExprError {
    UnexpectedEnd { pos: usize },
    UnexpectedChar { pos: usize, ch: char },
    Expected { pos: usize, expected: &'static str },
    UnknownFunction { pos: usize, name: String },
    InvalidRole { pos: usize, name: String },
    InvalidNumber { pos: usize, text: String },
    WrongArity { pos: usize, func: String, expected: usize },
}

impl std::fmt::Display for ValueExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueExprError::UnexpectedEnd { pos } => write!(f, "unexpected end of input at {pos}"),
            ValueExprError::UnexpectedChar { pos, ch } => {
                write!(f, "unexpected character '{ch}' at {pos}")
            }
            ValueExprError::Expected { pos, expected } => {
                write!(f, "expected {expected} at {pos}")
            }
            ValueExprError::UnknownFunction { pos, name } => {
                write!(f, "unknown function '{name}' at {pos}")
            }
            ValueExprError::InvalidRole { pos, name } => {
                write!(f, "invalid role '{name}' at {pos} (expected caster|target)")
            }
            ValueExprError::InvalidNumber { pos, text } => {
                write!(f, "invalid number '{text}' at {pos}")
            }
            ValueExprError::WrongArity { pos, func, expected } => {
                write!(f, "function '{func}' expects {expected} argument(s) at {pos}")
            }
        }
    }
}

impl std::error::Error for ValueExprError {}

/// Parse a value expression into a [`ValueDef`].
pub fn parse_value_expr(input: &str) -> Result<ValueDef, ValueExprError> {
    let mut parser = Parser::new(input);
    let value = parser.parse_sum()?;
    parser.skip_ws();
    match parser.peek() {
        None => Ok(value),
        Some((pos, ch)) => Err(ValueExprError::UnexpectedChar { pos, ch }),
    }
}

struct Parser<'a> {
    chars: Vec<(usize, char)>,
    idx: usize,
    _src: &'a str,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            chars: src.char_indices().collect(),
            idx: 0,
            _src: src,
        }
    }

    fn peek(&self) -> Option<(usize, char)> {
        self.chars.get(self.idx).copied()
    }

    fn next(&mut self) -> Option<(usize, char)> {
        let c = self.peek();
        if c.is_some() {
            self.idx += 1;
        }
        c
    }

    fn pos(&self) -> usize {
        self.peek().map(|(p, _)| p).unwrap_or(usize::MAX)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some((_, c)) if c.is_whitespace()) {
            self.idx += 1;
        }
    }

    fn eat(&mut self, want: char) -> bool {
        self.skip_ws();
        if matches!(self.peek(), Some((_, c)) if c == want) {
            self.idx += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, want: char, expected: &'static str) -> Result<(), ValueExprError> {
        if self.eat(want) {
            Ok(())
        } else {
            Err(ValueExprError::Expected {
                pos: self.pos(),
                expected,
            })
        }
    }

    fn parse_sum(&mut self) -> Result<ValueDef, ValueExprError> {
        let mut acc = self.parse_product()?;
        loop {
            if self.eat('+') {
                acc = ValueDef::Add(Box::new(acc), Box::new(self.parse_product()?));
            } else if self.eat('-') {
                acc = ValueDef::Sub(Box::new(acc), Box::new(self.parse_product()?));
            } else {
                return Ok(acc);
            }
        }
    }

    fn parse_product(&mut self) -> Result<ValueDef, ValueExprError> {
        let mut acc = self.parse_unary()?;
        loop {
            if self.eat('*') {
                acc = ValueDef::Multiply(Box::new(acc), Box::new(self.parse_unary()?));
            } else if self.eat('/') {
                acc = ValueDef::Divide(Box::new(acc), Box::new(self.parse_unary()?));
            } else {
                return Ok(acc);
            }
        }
    }

    fn parse_unary(&mut self) -> Result<ValueDef, ValueExprError> {
        self.skip_ws();
        if self.eat('-') {
            // Sub(0, x): saturating u16 semantics clamp the result at 0.
            Ok(ValueDef::Sub(
                Box::new(ValueDef::Constant(0)),
                Box::new(self.parse_unary()?),
            ))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<ValueDef, ValueExprError> {
        self.skip_ws();
        let Some((pos, ch)) = self.peek() else {
            return Err(ValueExprError::UnexpectedEnd { pos: usize::MAX });
        };
        if ch.is_ascii_digit() {
            return self.parse_number();
        }
        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.parse_call();
        }
        if ch == '(' {
            self.next();
            let inner = self.parse_sum()?;
            self.expect(')', "')'")?;
            return Ok(inner);
        }
        Err(ValueExprError::UnexpectedChar { pos, ch })
    }

    fn parse_number(&mut self) -> Result<ValueDef, ValueExprError> {
        let start = self.pos();
        let mut text = String::new();
        while let Some((_, c)) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            text.push(c);
            self.next();
        }
        text.parse::<u16>()
            .map(ValueDef::Constant)
            .map_err(|_| ValueExprError::InvalidNumber { pos: start, text })
    }

    fn parse_ident(&mut self) -> (usize, String) {
        let start = self.pos();
        let mut name = String::new();
        while let Some((_, c)) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                break;
            }
            name.push(c);
            self.next();
        }
        (start, name)
    }

    fn parse_call(&mut self) -> Result<ValueDef, ValueExprError> {
        let (pos, name) = self.parse_ident();
        self.expect('(', "'(' after function name")?;

        match name.as_str() {
            "attack" | "health" | "max_health" | "speed" => {
                let role = self.parse_role()?;
                self.expect(')', "')'")?;
                let stat = match name.as_str() {
                    "attack" => StatType::Attack,
                    "health" => StatType::Health,
                    "max_health" => StatType::MaxHealth,
                    "speed" => StatType::Speed,
                    _ => unreachable!(),
                };
                Ok(ValueDef::CreatureStat {
                    selector: Box::new(role_selector(role)),
                    stat,
                })
            }
            "min" | "max" | "random" => {
                let first = self.parse_sum()?;
                if !self.eat(',') {
                    return Err(ValueExprError::WrongArity {
                        pos: self.pos(),
                        func: name,
                        expected: 2,
                    });
                }
                let second = self.parse_sum()?;
                self.expect(')', "')'")?;
                let (a, b) = (Box::new(first), Box::new(second));
                Ok(match name.as_str() {
                    "min" => ValueDef::Min(a, b),
                    "max" => ValueDef::Max(a, b),
                    "random" => ValueDef::Random { min: a, max: b },
                    _ => unreachable!(),
                })
            }
            _ => Err(ValueExprError::UnknownFunction { pos, name }),
        }
    }

    fn parse_role(&mut self) -> Result<Role, ValueExprError> {
        self.skip_ws();
        let (pos, name) = self.parse_ident();
        match name.as_str() {
            "caster" => Ok(Role::Caster),
            "target" => Ok(Role::Target),
            _ => Err(ValueExprError::InvalidRole { pos, name }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Caster,
    Target,
}

fn role_selector(role: Role) -> SelectorDef {
    SelectorDef {
        kind: SelectorKindDef::Creature,
        cardinality: CardinalityDef::Single,
        selection: match role {
            Role::Caster => SelectionDef::Caster,
            Role::Target => SelectionDef::CurrentTarget,
        },
        filters: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn const_(v: u16) -> ValueDef {
        ValueDef::Constant(v)
    }

    #[test]
    fn precedence_mul_over_add() {
        let parsed = parse_value_expr("1+2*3").unwrap();
        assert_eq!(
            parsed,
            ValueDef::Add(
                Box::new(const_(1)),
                Box::new(ValueDef::Multiply(Box::new(const_(2)), Box::new(const_(3))))
            )
        );
    }

    #[test]
    fn parens_override_precedence() {
        let parsed = parse_value_expr("(1+2)*3").unwrap();
        assert_eq!(
            parsed,
            ValueDef::Multiply(
                Box::new(ValueDef::Add(Box::new(const_(1)), Box::new(const_(2)))),
                Box::new(const_(3))
            )
        );
    }

    #[test]
    fn nested_calls_and_sub() {
        let parsed = parse_value_expr("min(4-1, max(2, 3))").unwrap();
        assert_eq!(
            parsed,
            ValueDef::Min(
                Box::new(ValueDef::Sub(Box::new(const_(4)), Box::new(const_(1)))),
                Box::new(ValueDef::Max(Box::new(const_(2)), Box::new(const_(3))))
            )
        );
    }

    #[test]
    fn unary_minus_desugars_to_sub_zero() {
        let parsed = parse_value_expr("-5").unwrap();
        assert_eq!(
            parsed,
            ValueDef::Sub(Box::new(const_(0)), Box::new(const_(5)))
        );
        let parsed = parse_value_expr("--5").unwrap();
        assert_eq!(
            parsed,
            ValueDef::Sub(
                Box::new(const_(0)),
                Box::new(ValueDef::Sub(Box::new(const_(0)), Box::new(const_(5))))
            )
        );
    }

    #[test]
    fn stat_functions() {
        for (src, stat) in [
            ("attack(caster)", StatType::Attack),
            ("health(target)", StatType::Health),
            ("max_health(caster)", StatType::MaxHealth),
            ("speed(target)", StatType::Speed),
        ] {
            let parsed = parse_value_expr(src).unwrap();
            let ValueDef::CreatureStat { stat: got, .. } = parsed else {
                panic!("{src} did not parse to CreatureStat");
            };
            assert_eq!(got, stat);
        }
        assert!(matches!(
            parse_value_expr("random(1, 6)").unwrap(),
            ValueDef::Random { .. }
        ));
    }

    #[test]
    fn division_parses() {
        let parsed = parse_value_expr("attack(caster)/2").unwrap();
        assert!(matches!(parsed, ValueDef::Divide(_, _)));
    }

    #[test]
    fn errors() {
        assert!(matches!(
            parse_value_expr(""),
            Err(ValueExprError::UnexpectedEnd { .. })
        ));
        assert!(matches!(
            parse_value_expr("1+"),
            Err(ValueExprError::UnexpectedEnd { .. })
        ));
        assert!(matches!(
            parse_value_expr("foo(1)"),
            Err(ValueExprError::UnknownFunction { .. })
        ));
        assert!(matches!(
            parse_value_expr("attack(foo)"),
            Err(ValueExprError::InvalidRole { .. })
        ));
        assert!(matches!(
            parse_value_expr("(1"),
            Err(ValueExprError::Expected { .. })
        ));
        assert!(matches!(
            parse_value_expr("min(1)"),
            Err(ValueExprError::WrongArity { .. })
        ));
    }
}
