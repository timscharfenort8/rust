use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Target};

pub fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.endian = Endian::Big;
    base.cpu = cow!("v9");
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: cow!("sparc64-unknown-linux-gnu"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-i64:64-n32:64-S128"),
        arch: cow!("sparc64"),
        options: base,
    }
}
