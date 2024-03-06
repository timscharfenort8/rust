// Targets the Cortex-M33 processor (Armv8-M Mainline architecture profile),
use crate::spec::cow;

// with the Floating Point extension.

use crate::spec::{base, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("thumbv8m.main-none-eabihf"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            abi: cow!("eabihf"),
            // If the Floating Point extension is implemented in the Cortex-M33
            // processor, the Cortex-M33 Technical Reference Manual states that
            // the FPU uses the FPv5 architecture, single-precision instructions
            // and 16 D registers.
            // These parameters map to the following LLVM features.
            features: cow!("+fp-armv8,-fp64,-d32"),
            max_atomic_width: Some(32),
            ..base::thumb::opts()
        },
    }
};
