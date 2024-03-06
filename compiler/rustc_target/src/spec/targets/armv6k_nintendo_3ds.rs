use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::{cvs, Cc, LinkerFlavor, Lld, RelocModel, Target, TargetOptions};

/// A base target for Nintendo 3DS devices using the devkitARM toolchain.
///
/// Requires the devkitARM toolchain for 3DS targets on the host system.

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("armv6k-none-eabihf"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),

        options: TargetOptions {
            os: cow!("horizon"),
            env: cow!("newlib"),
            vendor: cow!("nintendo"),
            abi: cow!("eabihf"),
            cpu: cow!("mpcore"),
            families: cvs!["unix"],
            linker: Some(cow!("arm-none-eabi-gcc")),
            relocation_model: RelocModel::Static,
            features: cow!("+vfp2"),
            pre_link_args: LazyLock::new(|| {
                TargetOptions::link_args(
                    LinkerFlavor::Gnu(Cc::Yes, Lld::No),
                    &["-specs=3dsx.specs", "-mtune=mpcore", "-mfloat-abi=hard", "-mtp=soft"],
                )
            }),
            exe_suffix: cow!(".elf"),
            no_default_libraries: false,
            has_thread_local: true,
            ..TargetOptions::default()
        },
    }
};
