use crate::spec::cow;
use crate::spec::{base, StackProbeType, Target};

pub static TARGET: Target = {
    let mut base = base::redox::opts();
    base.max_atomic_width = Some(128);
    base.stack_probes = StackProbeType::Inline;
    base.features = cow!("+v8a");

    Target {
        llvm_target: cow!("aarch64-unknown-redox"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: base,
    }
};
