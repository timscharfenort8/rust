use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Cc, LinkerFlavor, Lld, Target, TargetOptions};
use std::sync::LazyLock;

pub fn target() -> Target {
    let mut base = base::openbsd::opts();
    base.endian = Endian::Big;
    base.cpu = cow!("v9");
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]));
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: cow!("sparc64-unknown-openbsd"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-i64:64-n32:64-S128"),
        arch: cow!("sparc64"),
        options: base,
    }
}
