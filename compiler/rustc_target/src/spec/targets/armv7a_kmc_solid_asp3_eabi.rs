use crate::spec::cow;
use crate::spec::{base, RelocModel, Target, TargetOptions};

pub static TARGET: Target = {
    let base = base::solid::opts("asp3");
    Target {
        llvm_target: cow!("armv7a-none-eabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),
        options: TargetOptions {
            linker: Some(cow!("arm-kmc-eabi-gcc")),
            features: cow!("+v7,+soft-float,+thumb2,-neon"),
            relocation_model: RelocModel::Static,
            disable_redzone: true,
            max_atomic_width: Some(64),
            ..base
        },
    }
};
