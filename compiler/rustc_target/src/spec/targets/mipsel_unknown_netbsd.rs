use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Target, TargetOptions};

pub static TARGET: Target = {
    let mut base = base::netbsd::opts();
    base.max_atomic_width = Some(32);
    base.cpu = cow!("mips32");

    Target {
        llvm_target: cow!("mipsel-unknown-netbsd"),
        pointer_width: 32,
        data_layout: cow!("e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: cow!("mips"),
        options: TargetOptions {
            features: cow!("+soft-float"),
            mcount: cow!("__mcount"),
            endian: Endian::Little,
            ..base
        },
    }
};
