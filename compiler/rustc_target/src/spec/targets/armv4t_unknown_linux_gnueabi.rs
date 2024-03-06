use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("armv4t-unknown-linux-gnueabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),
        options: TargetOptions {
            abi: cow!("eabi"),
            features: cow!("+soft-float,+strict-align"),
            // Atomic operations provided by compiler-builtins
            max_atomic_width: Some(32),
            mcount: cow!("\u{1}__gnu_mcount_nc"),
            llvm_mcount_intrinsic: Some(cow!("llvm.arm.gnu.eabi.mcount")),
            has_thumb_interworking: true,
            ..base::linux_gnu::opts()
        },
    }
};
