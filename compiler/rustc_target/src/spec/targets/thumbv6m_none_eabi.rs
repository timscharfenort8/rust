// Targets the Cortex-M0, Cortex-M0+ and Cortex-M1 processors (ARMv6-M architecture)
use crate::spec::cow;

use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("thumbv6m-none-eabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            abi: cow!("eabi"),
            // The ARMv6-M architecture doesn't support unaligned loads/stores so we disable them
            // with +strict-align.
            // Also force-enable 32-bit atomics, which allows the use of atomic load/store only.
            // The resulting atomics are ABI incompatible with atomics backed by libatomic.
            features: cow!("+strict-align,+atomics-32"),
            // There are no atomic CAS instructions available in the instruction set of the ARMv6-M
            // architecture
            atomic_cas: false,
            ..base::thumb::opts()
        },
    }
}
