use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};
use std::sync::LazyLock;

pub static TARGET: Target = {
    let mut base = base::linux_gnu::opts();
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-mspe"]));
    base.max_atomic_width = Some(32);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("powerpc-unknown-linux-gnuspe"),
        pointer_width: 32,
        data_layout: cow!("E-m:e-p:32:32-Fn32-i64:64-n32"),
        arch: cow!("powerpc"),
        options: TargetOptions {
            abi: cow!("spe"),
            endian: Endian::Big,
            mcount: cow!("_mcount"),
            ..base
        },
    }
};
