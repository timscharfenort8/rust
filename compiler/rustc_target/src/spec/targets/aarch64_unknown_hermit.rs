use crate::spec::cow;
use crate::spec::{base, StackProbeType, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("aarch64-unknown-hermit"),
        pointer_width: 64,
        arch: cow!("aarch64"),
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        options: TargetOptions {
            features: cow!("+v8a,+strict-align,+neon,+fp-armv8"),
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            ..base::hermit::opts()
        },
    }
};
