use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        // It's important we use "gnueabihf" and not "musleabihf" here. LLVM
        // uses it to determine the calling convention and float ABI, and it
        // doesn't support the "musleabihf" value.
        llvm_target: cow!("arm-unknown-linux-gnueabihf"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),
        options: TargetOptions {
            abi: cow!("eabihf"),
            // Most of these settings are copied from the arm_unknown_linux_gnueabihf
            // target.
            features: cow!("+strict-align,+v6,+vfp2,-d32"),
            max_atomic_width: Some(64),
            mcount: cow!("\u{1}mcount"),
            ..base::linux_musl::opts()
        },
    }
};
