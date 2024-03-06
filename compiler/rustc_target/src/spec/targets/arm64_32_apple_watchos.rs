use crate::spec::base::apple::{opts, Arch};
use crate::spec::cow;

use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    let base = opts("watchos", Arch::Arm64_32);
    Target {
        llvm_target: cow!("arm64_32-apple-watchos"),
        pointer_width: 32,
        data_layout: cow!("e-m:o-p:32:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            features: cow!("+v8a,+neon,+fp-armv8,+apple-a7"),
            max_atomic_width: Some(128),
            dynamic_linking: false,
            position_independent_executables: true,
            ..base
        },
    }
}
