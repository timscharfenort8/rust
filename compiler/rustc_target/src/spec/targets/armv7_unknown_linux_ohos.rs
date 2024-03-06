use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

// This target is for OpenHarmony on ARMv7 Linux with thumb-mode, but no NEON or
// hardfloat.

pub fn target() -> Target {
    // Most of these settings are copied from the armv7_unknown_linux_musleabi
    // target.
    Target {
        // LLVM 15 doesn't support OpenHarmony yet, use a linux target instead.
        llvm_target: cow!("armv7-unknown-linux-gnueabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            abi: cow!("eabi"),
            features: cow!("+v7,+thumb2,+soft-float,-neon"),
            max_atomic_width: Some(64),
            mcount: cow!("\u{1}mcount"),
            ..base::linux_ohos::opts()
        },
    }
}
