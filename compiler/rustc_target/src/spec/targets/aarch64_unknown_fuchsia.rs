use crate::spec::cow;
use crate::spec::{base, SanitizerSet, StackProbeType, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("aarch64-unknown-fuchsia"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            features: cow!("+v8a"),
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            supported_sanitizers: SanitizerSet::ADDRESS
                | SanitizerSet::CFI
                | SanitizerSet::SHADOWCALLSTACK,
            ..base::fuchsia::opts()
        },
    }
};
