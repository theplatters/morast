use janet_bindings::{controller::CoreConstant, types::janetenum::JanetEnum};

pub const CORE_CONSTANTS: [CoreConstant; 6] = [
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
