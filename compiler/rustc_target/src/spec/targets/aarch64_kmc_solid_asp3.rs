use crate::spec::cow;
use crate::spec::{base, RelocModel, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let base = base::solid::opts("asp3");
    Target {
        llvm_target: cow!("aarch64-unknown-none"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            linker: Some(cow!("aarch64-kmc-elf-gcc")),
            features: cow!("+v8a,+neon,+fp-armv8"),
            relocation_model: RelocModel::Static,
            disable_redzone: true,
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            ..base
        },
    }
}
