use crate::spec::cow;
use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel};

use crate::spec::{Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        data_layout: cow!("e-m:e-p:32:32-i64:64-n32-S128"),
        llvm_target: cow!("riscv32"),
        pointer_width: 32,
        arch: cow!("riscv32"),

        options: TargetOptions {
            os: cow!("zkvm"),
            vendor: cow!("risc0"),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some(cow!("rust-lld")),
            cpu: cow!("generic-rv32"),

            // Some crates (*cough* crossbeam) assume you have 64 bit
            // atomics if the target name is not in a hardcoded list.
            // Since zkvm is singlethreaded and all operations are
            // atomic, I guess we can just say we support 64-bit
            // atomics.
            max_atomic_width: Some(64),
            atomic_cas: true,

            features: cow!("+m"),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            singlethread: true,
            ..TargetOptions::default()
        },
    }
};
