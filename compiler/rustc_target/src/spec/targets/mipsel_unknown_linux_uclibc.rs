use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("mipsel-unknown-linux-uclibc"),
        pointer_width: 32,
        data_layout: cow!("e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: cow!("mips"),

        options: TargetOptions {
            cpu: cow!("mips32r2"),
            features: cow!("+mips32r2,+soft-float"),
            max_atomic_width: Some(32),
            mcount: cow!("_mcount"),

            ..base::linux_uclibc::opts()
        },
    }
}
