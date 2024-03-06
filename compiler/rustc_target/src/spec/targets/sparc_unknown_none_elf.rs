use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub static TARGET: Target = {
    let options = TargetOptions {
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        linker: Some(cow!("sparc-elf-gcc")),
        endian: Endian::Big,
        cpu: cow!("v7"),
        abi: cow!("elf"),
        max_atomic_width: Some(32),
        atomic_cas: true,
        panic_strategy: PanicStrategy::Abort,
        relocation_model: RelocModel::Static,
        no_default_libraries: false,
        emit_debug_gdb_scripts: false,
        eh_frame_header: false,
        ..TargetOptions::default()
    };
    Target {
        data_layout: cow!("E-m:e-p:32:32-i64:64-f128:64-n32-S64"),
        llvm_target: cow!("sparc-unknown-none-elf"),
        pointer_width: 32,
        arch: cow!("sparc"),
        options,
    }
};
