use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Cc, LinkerFlavor, Target, TargetOptions};
use std::sync::LazyLock;

pub fn target() -> Target {
    let mut base = base::solaris::opts();
    base.endian = Endian::Big;
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Unix(Cc::Yes), &["-m64"]));
    // llvm calls this "v9"
    base.cpu = cow!("v9");
    base.vendor = cow!("sun");
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: cow!("sparcv9-sun-solaris"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-i64:64-n32:64-S128"),
        // Use "sparc64" instead of "sparcv9" here, since the former is already
        // used widely in the source base. If we ever needed ABI
        // differentiation from the sparc64, we could, but that would probably
        // just be confusing.
        arch: cow!("sparc64"),
        options: base,
    }
}
