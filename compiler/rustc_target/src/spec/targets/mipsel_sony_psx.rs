use crate::spec::cow;
use crate::spec::{cvs, Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("mipsel-sony-psx"),
        pointer_width: 32,
        data_layout: cow!("e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: cow!("mips"),

        options: TargetOptions {
            os: cow!("none"),
            env: cow!("psx"),
            vendor: cow!("sony"),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            cpu: cow!("mips1"),
            executables: true,
            linker: Some(cow!("rust-lld")),
            relocation_model: RelocModel::Static,
            exe_suffix: cow!(".exe"),

            // PSX doesn't natively support floats.
            features: cow!("+soft-float"),

            // This should be 16 bits, but LLVM incorrectly tries emitting MIPS-II SYNC instructions
            // for atomic loads and stores. This crashes rustc so we have to disable the Atomic* API
            // until this is fixed upstream. See https://reviews.llvm.org/D122427#3420144 for more
            // info.
            max_atomic_width: Some(0),

            // PSX does not support trap-on-condition instructions.
            llvm_args: cvs!["-mno-check-zero-division"],
            llvm_abiname: cow!("o32"),
            panic_strategy: PanicStrategy::Abort,
            ..TargetOptions::default()
        },
    }
}
