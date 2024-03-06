use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Target, TargetOptions};

pub static TARGET: Target = {
    let mut base = base::linux_musl::opts();
    base.cpu = cow!("mips64r2");
    base.features = cow!("+mips64r2");
    base.max_atomic_width = Some(64);
    Target {
        // LLVM doesn't recognize "muslabi64" yet.
        llvm_target: cow!("mips64-unknown-linux-musl"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-i8:8:32-i16:16:32-i64:64-n32:64-S128"),
        arch: cow!("mips64"),
        options: TargetOptions {
            abi: cow!("abi64"),
            endian: Endian::Big,
            mcount: cow!("_mcount"),
            ..base
        },
    }
};
