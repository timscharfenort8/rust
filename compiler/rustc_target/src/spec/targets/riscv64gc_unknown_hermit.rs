use crate::spec::cow;
use crate::spec::{base, CodeModel, RelocModel, Target, TargetOptions, TlsModel};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("riscv64-unknown-hermit"),
        pointer_width: 64,
        arch: cow!("riscv64"),
        data_layout: cow!("e-m:e-p:64:64-i64:64-i128:128-n32:64-S128"),
        options: TargetOptions {
            cpu: cow!("generic-rv64"),
            features: cow!("+m,+a,+f,+d,+c"),
            relocation_model: RelocModel::Pic,
            code_model: Some(CodeModel::Medium),
            tls_model: TlsModel::LocalExec,
            max_atomic_width: Some(64),
            llvm_abiname: cow!("lp64d"),
            ..base::hermit::opts()
        },
    }
};
