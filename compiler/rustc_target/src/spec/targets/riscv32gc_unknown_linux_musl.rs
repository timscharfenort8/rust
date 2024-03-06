use crate::spec::cow;
use std::borrow::Cow;

use crate::spec::{base, CodeModel, SplitDebuginfo, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("riscv32-unknown-linux-musl"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-i64:64-n32-S128"),
        arch: cow!("riscv32"),
        options: TargetOptions {
            code_model: Some(CodeModel::Medium),
            cpu: cow!("generic-rv32"),
            features: cow!("+m,+a,+f,+d,+c"),
            llvm_abiname: cow!("ilp32d"),
            max_atomic_width: Some(32),
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
            ..base::linux_musl::opts()
        },
    }
};
