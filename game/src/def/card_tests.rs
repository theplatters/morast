//! Integration test: every RON card file under `assets/cards` must deserialize
//! into a [`CardDef`], and its `name` must match the file's stem. This is the
//! "all current cards translated" gate for WP4.

#[cfg(test)]
mod card_tests {
    use crate::def::card::{CardDef, CardKindDef};
    use std::path::{Path, PathBuf};

    /// Path to `assets/cards` relative to the crate manifest.
    fn cards_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("assets").join("cards")
    }

    #[test]
    fn all_cards_parse_and_match_filename() {
        let dir = cards_dir();
        assert!(
            dir.exists(),
            "assets/cards directory missing at {:?}",
            dir
        );

        let mut parsed_any = false;
        for entry in std::fs::read_dir(&dir)
            .expect("read assets/cards")
        {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("ron") {
                continue;
            }
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("file stem")
                .to_string();

            let src = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {:?}: {}", path, e));

            let card: CardDef = ron::de::from_str(&src)
                .unwrap_or_else(|e| panic!("failed to parse {:?}: {}", path, e));

            assert_eq!(
                card.name, stem,
                "card name {:?} does not match filename stem {:?}",
                card.name, stem
            );

            // A card must have a sensible (non-zero) cost and a valid kind.
            assert!(card.cost > 0, "card {:?} has zero cost", card.name);
            match &card.kind {
                CardKindDef::Creature(stats) => {
                    assert!(stats.attack > 0, "creature {:?} attack 0", card.name);
                    assert!(stats.defense > 0, "creature {:?} defense 0", card.name);
                }
                CardKindDef::Spell | CardKindDef::Trap => {}
            }

            parsed_any = true;
            eprintln!("parsed card {:?} (cost {})", card.name, card.cost);
        }

        assert!(parsed_any, "no .ron cards found in {:?}", dir);
    }
}
