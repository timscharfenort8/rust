use crate::spec::cow;
use crate::spec::{cvs, PanicStrategy, RelocModel, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        data_layout: cow!("e-m:e-p:32:32-i64:64-n32-S128"),
        llvm_target: cow!("riscv32"),
        pointer_width: 32,
        arch: cow!("riscv32"),

        options: TargetOptions {
            families: cvs!["unix"],
            os: cow!("espidf"),
            env: cow!("newlib"),
            vendor: cow!("espressif"),
            linker: Some(cow!("riscv32-esp-elf-gcc")),
            cpu: cow!("generic-rv32"),

            max_atomic_width: Some(32),
            atomic_cas: true,

            llvm_abiname: cow!("ilp32f"),
            features: cow!("+m,+a,+c,+f"),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..TargetOptions::default()
        },
    }
};
