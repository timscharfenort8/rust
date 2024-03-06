use crate::spec::cow;
use std::borrow::Cow;

use crate::spec::{base, CodeModel, SplitDebuginfo, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("riscv64-unknown-linux-gnu"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-p:64:64-i64:64-i128:128-n32:64-S128"),
        arch: cow!("riscv64"),
        options: TargetOptions {
            code_model: Some(CodeModel::Medium),
            cpu: cow!("generic-rv64"),
            features: cow!("+m,+a,+f,+d,+c"),
            llvm_abiname: cow!("lp64d"),
            max_atomic_width: Some(64),
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
            ..base::linux_gnu::opts()
        },
    }
};
