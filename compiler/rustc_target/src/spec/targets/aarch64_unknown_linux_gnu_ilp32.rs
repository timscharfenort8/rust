use crate::spec::cow;
use crate::spec::{base, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("aarch64-unknown-linux-gnu_ilp32"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            abi: cow!("ilp32"),
            features: cow!("+v8a,+outline-atomics"),
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            mcount: cow!("\u{1}_mcount"),
            ..base::linux_gnu::opts()
        },
    }
}
