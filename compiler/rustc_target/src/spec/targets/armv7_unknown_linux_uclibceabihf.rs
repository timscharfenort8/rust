use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

// This target is for uclibc Linux on ARMv7 without NEON or
// thumb-mode. See the thumbv7neon variant for enabling both.

pub fn target() -> Target {
    let base = base::linux_uclibc::opts();
    Target {
        llvm_target: cow!("armv7-unknown-linux-gnueabihf"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            // Info about features at https://wiki.debian.org/ArmHardFloatPort
            features: cow!("+v7,+vfp3,-d32,+thumb2,-neon"),
            cpu: cow!("generic"),
            max_atomic_width: Some(64),
            mcount: cow!("_mcount"),
            abi: cow!("eabihf"),
            ..base
        },
    }
}
