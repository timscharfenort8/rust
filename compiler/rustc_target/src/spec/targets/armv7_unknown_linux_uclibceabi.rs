use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

// This target is for uclibc Linux on ARMv7 without NEON,
// thumb-mode or hardfloat.

pub static TARGET: Target = {
    let base = base::linux_uclibc::opts();
    Target {
        llvm_target: cow!("armv7-unknown-linux-gnueabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            features: cow!("+v7,+thumb2,+soft-float,-neon"),
            cpu: cow!("generic"),
            max_atomic_width: Some(64),
            mcount: cow!("_mcount"),
            abi: cow!("eabi"),
            ..base
        },
    }
};
