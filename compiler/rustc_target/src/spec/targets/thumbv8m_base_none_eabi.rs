// Targets the Cortex-M23 processor (Baseline ARMv8-M)
use crate::spec::cow;

use crate::spec::{base, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("thumbv8m.base-none-eabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            abi: cow!("eabi"),
            // ARMv8-M baseline doesn't support unaligned loads/stores so we disable them
            // with +strict-align.
            features: cow!("+strict-align"),
            max_atomic_width: Some(32),
            ..base::thumb::opts()
        },
    }
};
