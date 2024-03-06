use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Cc, LinkerFlavor, Lld, Target, TargetOptions};
use std::sync::LazyLock;

pub fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.endian = Endian::Big;
    base.cpu = cow!("v9");
    base.max_atomic_width = Some(32);
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-mv8plus"])
    });

    Target {
        llvm_target: cow!("sparc-unknown-linux-gnu"),
        pointer_width: 32,
        data_layout: cow!("E-m:e-p:32:32-i64:64-f128:64-n32-S64"),
        arch: cow!("sparc"),
        options: base,
    }
}
