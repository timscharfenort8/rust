use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.max_atomic_width = Some(128);

    Target {
        llvm_target: cow!("aarch64_be-unknown-linux-gnu_ilp32"),
        pointer_width: 32,
        data_layout: cow!("E-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            abi: cow!("ilp32"),
            features: cow!("+v8a,+outline-atomics"),
            stack_probes: StackProbeType::Inline,
            mcount: cow!("\u{1}_mcount"),
            endian: Endian::Big,
            ..base
        },
    }
}
