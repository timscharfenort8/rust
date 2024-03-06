use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.cpu = cow!("M68020");
    base.max_atomic_width = Some(32);

    Target {
        llvm_target: cow!("m68k-unknown-linux-gnu"),
        pointer_width: 32,
        data_layout: cow!("E-m:e-p:32:16:32-i8:8:8-i16:16:16-i32:16:32-n8:16:32-a:0:16-S16"),
        arch: cow!("m68k"),
        options: TargetOptions { endian: Endian::Big, mcount: cow!("_mcount"), ..base },
    }
}
