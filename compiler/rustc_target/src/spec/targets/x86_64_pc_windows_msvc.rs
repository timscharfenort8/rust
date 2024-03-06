use crate::spec::cow;
use crate::spec::{base, SanitizerSet, Target};

pub static TARGET: Target = {
    let mut base = base::windows_msvc::opts();
    base.cpu = cow!("x86-64");
    base.features = cow!("+cx16,+sse3,+sahf");
    base.plt_by_default = false;
    base.max_atomic_width = Some(128);
    base.supported_sanitizers = SanitizerSet::ADDRESS;

    Target {
        llvm_target: cow!("x86_64-pc-windows-msvc"),
        pointer_width: 64,
        data_layout: cow!(
            "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        arch: cow!("x86_64"),
        options: base,
    }
};
