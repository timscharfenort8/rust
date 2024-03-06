use crate::spec::cow;
use crate::spec::{base, Cc, LinkerFlavor, SanitizerSet, StackProbeType, Target, TargetOptions};

use std::sync::LazyLock;

pub fn target() -> Target {
    let mut base = base::solaris::opts();
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Unix(Cc::Yes), &["-m64"]));
    base.cpu = cow!("x86-64");
    base.plt_by_default = false;
    base.vendor = cow!("pc");
    base.max_atomic_width = Some(64);
    base.stack_probes = StackProbeType::Inline;
    base.supported_sanitizers = SanitizerSet::ADDRESS | SanitizerSet::CFI | SanitizerSet::THREAD;

    Target {
        llvm_target: cow!("x86_64-pc-solaris"),
        pointer_width: 64,
        data_layout: cow!(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        arch: cow!("x86_64"),
        options: base,
    }
}
