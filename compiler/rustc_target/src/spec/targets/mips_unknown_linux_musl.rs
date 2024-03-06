use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = base::linux_musl::opts();
    base.cpu = cow!("mips32r2");
    base.features = cow!("+mips32r2,+soft-float");
    base.max_atomic_width = Some(32);
    base.crt_static_default = false;
    Target {
        llvm_target: cow!("mips-unknown-linux-musl"),
        pointer_width: 32,
        data_layout: cow!("E-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: cow!("mips"),
        options: TargetOptions { endian: Endian::Big, mcount: cow!("_mcount"), ..base },
    }
}
