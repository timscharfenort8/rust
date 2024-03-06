use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("mipsisa64r6-unknown-linux-gnuabi64"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-i8:8:32-i16:16:32-i64:64-n32:64-S128"),
        arch: cow!("mips64r6"),
        options: TargetOptions {
            abi: cow!("abi64"),
            endian: Endian::Big,
            // NOTE(mips64r6) matches C toolchain
            cpu: cow!("mips64r6"),
            features: cow!("+mips64r6"),
            max_atomic_width: Some(64),
            mcount: cow!("_mcount"),

            ..base::linux_gnu::opts()
        },
    }
}
