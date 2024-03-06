use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        // FIXME: this comment below does not seem applicable?
        // It's important we use "gnueabihf" and not "musleabihf" here. LLVM
        // uses it to determine the calling convention and float ABI, and LLVM
        // doesn't support the "musleabihf" value.
        llvm_target: cow!("armv5te-unknown-linux-gnueabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),
        options: TargetOptions {
            abi: cow!("eabi"),
            features: cow!("+soft-float,+strict-align"),
            // Atomic operations provided by compiler-builtins
            max_atomic_width: Some(32),
            mcount: cow!("\u{1}mcount"),
            has_thumb_interworking: true,
            ..base::linux_musl::opts()
        },
    }
}
