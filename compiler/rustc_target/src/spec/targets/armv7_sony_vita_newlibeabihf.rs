use crate::spec::cow;
use std::sync::LazyLock;

use crate::abi::Endian;
use crate::spec::{cvs, Cc, LinkerFlavor, Lld, RelocModel, Target, TargetOptions};

/// A base target for PlayStation Vita devices using the VITASDK toolchain (using newlib).
///
/// Requires the VITASDK toolchain on the host system.

pub fn target() -> Target {
    Target {
        llvm_target: cow!("thumbv7a-vita-eabihf"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            os: cow!("vita"),
            endian: Endian::Little,
            c_int_width: cow!("32"),
            env: cow!("newlib"),
            vendor: cow!("sony"),
            abi: cow!("eabihf"),
            linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            no_default_libraries: false,
            cpu: cow!("cortex-a9"),
            families: cvs!["unix"],
            linker: Some(cow!("arm-vita-eabi-gcc")),
            relocation_model: RelocModel::Static,
            features: cow!("+v7,+neon,+vfp3,+thumb2,+thumb-mode"),
            pre_link_args: LazyLock::new(|| {
                TargetOptions::link_args(
                    LinkerFlavor::Gnu(Cc::Yes, Lld::No),
                    &["-Wl,-q", "-Wl,--pic-veneer"],
                )
            }),
            exe_suffix: cow!(".elf"),
            has_thumb_interworking: true,
            max_atomic_width: Some(64),
            ..TargetOptions::default()
        },
    }
}
