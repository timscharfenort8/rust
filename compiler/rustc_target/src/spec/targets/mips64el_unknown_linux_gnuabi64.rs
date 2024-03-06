use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("mips64el-unknown-linux-gnuabi64"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-n32:64-S128"),
        arch: cow!("mips64"),
        options: TargetOptions {
            abi: cow!("abi64"),
            // NOTE(mips64r2) matches C toolchain
            cpu: cow!("mips64r2"),
            features: cow!("+mips64r2,+xgot"),
            max_atomic_width: Some(64),
            mcount: cow!("_mcount"),

            ..base::linux_gnu::opts()
        },
    }
}
