use crate::spec::cow;
use crate::spec::{base, StackProbeType, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("x86_64-unknown-hermit"),
        pointer_width: 64,
        arch: cow!("x86_64"),
        data_layout: cow!(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        options: TargetOptions {
            cpu: cow!("x86-64"),
            features: cow!("+rdrnd,+rdseed"),
            plt_by_default: false,
            max_atomic_width: Some(64),
            stack_probes: StackProbeType::Inline,
            ..base::hermit::opts()
        },
    }
};
