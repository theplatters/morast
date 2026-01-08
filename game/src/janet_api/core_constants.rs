use crate::engine::janet_handler::{controller::Environment, types::janetenum::JanetEnum};

pub struct CoreConstant {
    pub name: &'static str,
    pub value: JanetEnum,
    pub docs: Option<&'static str>,
}

pub const CORE_CONSTANTS: &[CoreConstant] = &[
    // Phase Constants
    CoreConstant {
        name: "phase/start",
        value: JanetEnum::Int(0),
        docs: Some(
            "Represents the start phase of execution. Used to indicate the beginning of a process or operation.",
        ),
    },
    CoreConstant {
        name: "phase/main",
        value: JanetEnum::Int(1),
        docs: Some(
            "Represents the main phase of execution. Used for the primary processing or core logic phase.",
        ),
    },
    CoreConstant {
        name: "phase/end",
        value: JanetEnum::Int(2),
        docs: Some(
            "Represents the end phase of execution. Used to indicate the completion or cleanup phase.",
        ),
    },
    // Phase Constants
    CoreConstant {
        name: "spell/slow",
        value: JanetEnum::Int(0),
        docs: Some(
            "Represents a slow spell speed, that can be countered by fast and instant spells",
        ),
    },
    CoreConstant {
        name: "spell/fast",
        value: JanetEnum::Int(1),
        docs: Some("Represents a spell speed, that can counter slow spells"),
    },
    CoreConstant {
        name: "spell/instant",
        value: JanetEnum::Int(2),
        docs: Some("Represents a spell speed that cannot be countered"),
    },
];

impl Environment {
    pub fn register_core_constants(&self) {
        for constant in CORE_CONSTANTS {
            self.register_constant(constant.name, &constant.value, constant.docs)
                .unwrap_or_else(|_| panic!("Could not register {} function", constant.name));
        }
    }
}
