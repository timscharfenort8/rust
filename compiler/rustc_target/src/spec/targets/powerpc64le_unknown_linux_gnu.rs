use crate::spec::cow;
use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};

use std::sync::LazyLock;

pub static TARGET: Target = {
    let mut base = base::linux_gnu::opts();
    base.cpu = cow!("ppc64le");
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]));
    base.max_atomic_width = Some(64);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("powerpc64le-unknown-linux-gnu"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-Fn32-i64:64-n32:64-S128-v256:256:256-v512:512:512"),
        arch: cow!("powerpc64"),
        options: TargetOptions { mcount: cow!("_mcount"), ..base },
    }
};
