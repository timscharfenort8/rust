use crate::spec::cow;
use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy};

use crate::spec::{RelocModel, SanitizerSet, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: cow!("e-m:e-p:64:64-i64:64-i128:128-n32:64-S128"),
        llvm_target: cow!("riscv64"),
        pointer_width: 64,
        arch: cow!("riscv64"),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some(cow!("rust-lld")),
            cpu: cow!("generic-rv64"),
            max_atomic_width: Some(64),
            features: cow!("+m,+a,+c"),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            code_model: Some(CodeModel::Medium),
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            supported_sanitizers: SanitizerSet::KERNELADDRESS,
            ..TargetOptions::default()
        },
    }
}
