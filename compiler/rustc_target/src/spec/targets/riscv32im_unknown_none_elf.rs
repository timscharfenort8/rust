use crate::spec::cow;
use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: cow!("e-m:e-p:32:32-i64:64-n32-S128"),
        llvm_target: cow!("riscv32"),
        pointer_width: 32,
        arch: cow!("riscv32"),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some(cow!("rust-lld")),
            cpu: cow!("generic-rv32"),
            max_atomic_width: Some(32),
            atomic_cas: false,
            features: cow!("+m,+forced-atomics"),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..TargetOptions::default()
        },
    }
}
