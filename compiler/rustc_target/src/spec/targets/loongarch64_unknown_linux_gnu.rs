use crate::spec::cow;
use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("loongarch64-unknown-linux-gnu"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-p:64:64-i64:64-i128:128-n64-S128"),
        arch: cow!("loongarch64"),
        options: TargetOptions {
            cpu: cow!("generic"),
            features: cow!("+f,+d"),
            llvm_abiname: cow!("lp64d"),
            max_atomic_width: Some(64),
            direct_access_external_data: Some(false),
            ..base::linux_gnu::opts()
        },
    }
}
