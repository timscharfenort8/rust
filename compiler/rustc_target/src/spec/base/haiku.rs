use crate::spec::cow;
use crate::spec::{cvs, RelroLevel, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: cow!("haiku"),
        dynamic_linking: true,
        families: cvs!["unix"],
        relro_level: RelroLevel::Full,
        ..TargetOptions::default()
    }
}
