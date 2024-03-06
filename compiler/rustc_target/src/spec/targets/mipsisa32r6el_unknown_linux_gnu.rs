use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("mipsisa32r6el-unknown-linux-gnu"),
        pointer_width: 32,
        data_layout: cow!("e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: cow!("mips32r6"),

        options: TargetOptions {
            cpu: cow!("mips32r6"),
            features: cow!("+mips32r6"),
            max_atomic_width: Some(32),
            mcount: cow!("_mcount"),

            ..base::linux_gnu::opts()
        },
    }
}
