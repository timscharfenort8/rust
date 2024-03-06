use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::TargetOptions;
use crate::spec::{base, Cc, LinkerFlavor, Target};

pub fn target() -> Target {
    let mut base = base::aix::opts();
    base.max_atomic_width = Some(64);
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(
            LinkerFlavor::Unix(Cc::No),
            &["-b64", "-bpT:0x100000000", "-bpD:0x110000000", "-bcdtors:all:0:s"],
        )
    });
    Target {
        llvm_target: cow!("powerpc64-ibm-aix"),
        pointer_width: 64,
        data_layout: cow!("E-m:a-Fi64-i64:64-n32:64-S128-v256:256:256-v512:512:512"),
        arch: cow!("powerpc64"),
        options: base,
    }
}
