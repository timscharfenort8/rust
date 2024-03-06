use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};
use std::sync::LazyLock;

pub fn target() -> Target {
    let mut base = base::freebsd::opts();
    base.cpu = cow!("ppc64");
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]));
    base.max_atomic_width = Some(64);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("powerpc64-unknown-freebsd"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-Fn32-i64:64-n32:64"),
        arch: cow!("powerpc64"),
        options: TargetOptions { endian: Endian::Big, mcount: cow!("_mcount"), ..base },
    }
}
