use crate::spec::cow;
use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelroLevel, StackProbeType, Target, TargetOptions,
};

const LINKER_SCRIPT: &str = include_str!("./aarch64_nintendo_switch_freestanding_linker_script.ld");

/// A base target for Nintendo Switch devices using a pure LLVM toolchain.
pub fn target() -> Target {
    Target {
        llvm_target: cow!("aarch64-unknown-none"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            features: cow!("+v8a"),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some(cow!("rust-lld")),
            link_script: Some(LINKER_SCRIPT.into()),
            os: cow!("horizon"),
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            panic_strategy: PanicStrategy::Abort,
            position_independent_executables: true,
            dynamic_linking: true,
            relro_level: RelroLevel::Off,
            ..TargetOptions::default()
        },
    }
}
