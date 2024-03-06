use crate::spec::cow;
use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy, RelocModel};

use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("loongarch64-unknown-none"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-p:64:64-i64:64-i128:128-n64-S128"),
        arch: cow!("loongarch64"),
        options: TargetOptions {
            cpu: cow!("generic"),
            features: cow!("-f,-d"),
            abi: cow!("softfloat"),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some(cow!("rust-lld")),
            llvm_abiname: cow!("lp64s"),
            max_atomic_width: Some(64),
            relocation_model: RelocModel::Static,
            panic_strategy: PanicStrategy::Abort,
            code_model: Some(CodeModel::Small),
            ..TargetOptions::default()
        },
    }
}
