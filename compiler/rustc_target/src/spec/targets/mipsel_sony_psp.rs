use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::{cvs, Cc, LinkerFlavor, Lld, RelocModel, Target, TargetOptions};

// The PSP has custom linker requirements.
const LINKER_SCRIPT: &str = include_str!("./mipsel_sony_psp_linker_script.ld");

pub fn target() -> Target {
    Target {
        llvm_target: cow!("mipsel-sony-psp"),
        pointer_width: 32,
        data_layout: cow!("e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: cow!("mips"),

        options: TargetOptions {
            os: cow!("psp"),
            vendor: cow!("sony"),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            cpu: cow!("mips2"),
            linker: Some(cow!("rust-lld")),
            relocation_model: RelocModel::Static,

            // PSP FPU only supports single precision floats.
            features: cow!("+single-float"),

            // PSP does not support trap-on-condition instructions.
            llvm_args: cvs!["-mno-check-zero-division"],
            pre_link_args: LazyLock::new(|| {
                TargetOptions::link_args(
                    LinkerFlavor::Gnu(Cc::No, Lld::No),
                    &["--emit-relocs", "--nmagic"],
                )
            }),
            link_script: Some(LINKER_SCRIPT.into()),
            ..TargetOptions::default()
        },
    }
}
