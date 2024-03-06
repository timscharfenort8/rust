//! Bare MIPS32r2, little endian, softfloat, O32 calling convention
//!
//! Can be used for MIPS M4K core (e.g. on PIC32MX devices)

use crate::spec::cow;
use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("mipsel-unknown-none"),
        pointer_width: 32,
        data_layout: cow!("e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: cow!("mips"),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            cpu: cow!("mips32r2"),
            features: cow!("+mips32r2,+soft-float,+noabicalls"),
            max_atomic_width: Some(32),
            linker: Some(cow!("rust-lld")),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            ..TargetOptions::default()
        },
    }
}
