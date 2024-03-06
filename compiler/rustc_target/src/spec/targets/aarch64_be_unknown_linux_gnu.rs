use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, StackProbeType, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("aarch64_be-unknown-linux-gnu"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            features: cow!("+v8a,+outline-atomics"),
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            mcount: cow!("\u{1}_mcount"),
            endian: Endian::Big,
            ..base::linux_gnu::opts()
        },
    }
};
