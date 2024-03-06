use crate::spec::cow;
use crate::spec::{cvs, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
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

            // While the RiscV32IMC architecture does not natively support atomics, ESP-IDF does support
            // the __atomic* and __sync* GCC builtins, so setting `max_atomic_width` to `Some(32)`
            // and `atomic_cas` to `true` will cause the compiler to emit libcalls to these builtins.
            //
            // Support for atomics is necessary for the Rust STD library, which is supported by the ESP-IDF framework.
            max_atomic_width: Some(32),
            atomic_cas: true,

            features: cow!("+m,+c"),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..TargetOptions::default()
        },
    }
}
