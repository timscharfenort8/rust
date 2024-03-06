use crate::spec::cow;
use crate::spec::{cvs, RelroLevel, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        crt_static_respected: true,
        dynamic_linking: true,
        executables: true,
        families: cvs!["unix"],
        has_rpath: true,
        has_thread_local: false,
        linker: Some(cow!("qcc")),
        os: cow!("nto"),
        position_independent_executables: true,
        static_position_independent_executables: true,
        relro_level: RelroLevel::Full,
        ..TargetOptions::default()
    }
}
