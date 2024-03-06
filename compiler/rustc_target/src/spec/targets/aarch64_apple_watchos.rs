use crate::spec::base::apple::{opts, Arch};
use crate::spec::cow;

use crate::spec::{Target, TargetOptions};

pub static TARGET: Target = {
    let base = opts("watchos", Arch::Arm64);
    Target {
        llvm_target: cow!("aarch64-apple-watchos"),
        pointer_width: 64,
        data_layout: cow!("e-m:o-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            features: cow!("+v8a,+neon,+fp-armv8,+apple-a7"),
            max_atomic_width: Some(128),
            dynamic_linking: false,
            position_independent_executables: true,
            ..base
        },
    }
};
