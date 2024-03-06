// Generic AArch64 target for bare-metal code - Floating point disabled
use crate::spec::cow;

//
// Can be used in conjunction with the `target-feature` and
// `target-cpu` compiler flags to opt-in more hardware-specific
// features.
//
// For example, `-C target-cpu=cortex-a53`.

use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, StackProbeType, Target, TargetOptions,
};

pub fn target() -> Target {
    let opts = TargetOptions {
        abi: cow!("softfloat"),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some(cow!("rust-lld")),
        features: cow!("+v8a,+strict-align,-neon,-fp-armv8"),
        relocation_model: RelocModel::Static,
        disable_redzone: true,
        max_atomic_width: Some(128),
        stack_probes: StackProbeType::Inline,
        panic_strategy: PanicStrategy::Abort,
        ..TargetOptions::default()
    };
    Target {
        llvm_target: cow!("aarch64-unknown-none"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: opts,
    }
}
