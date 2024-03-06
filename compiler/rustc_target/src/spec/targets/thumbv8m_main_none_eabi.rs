// Targets the Cortex-M33 processor (Armv8-M Mainline architecture profile),
use crate::spec::cow;

// without the Floating Point extension.

use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("thumbv8m.main-none-eabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            abi: cow!("eabi"),
            max_atomic_width: Some(32),
            ..base::thumb::opts()
        },
    }
}
