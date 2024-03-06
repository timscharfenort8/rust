use crate::spec::cow;
use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};

use std::sync::LazyLock;

pub static TARGET: Target = {
    let mut base = base::freebsd::opts();
    base.cpu = cow!("pentium4");
    base.max_atomic_width = Some(64);
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m32", "-Wl,-znotext"])
    });
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("i686-unknown-freebsd"),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i128:128-f64:32:64-f80:32-n8:16:32-S128"
            .into(),
        arch: cow!("x86"),
        options: base,
    }
};
