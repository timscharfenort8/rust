use crate::spec::cow;
use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};

use std::sync::LazyLock;

pub static TARGET: Target = {
    let mut base = base::redox::opts();
    base.cpu = cow!("x86-64");
    base.plt_by_default = false;
    base.max_atomic_width = Some(64);
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]));
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("x86_64-unknown-redox"),
        pointer_width: 64,
        data_layout: cow!(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        arch: cow!("x86_64"),
        options: base,
    }
};
