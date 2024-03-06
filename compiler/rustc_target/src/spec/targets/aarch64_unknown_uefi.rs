use crate::spec::cow;
use std::sync::LazyLock;

// This defines the aarch64 target for UEFI systems as described in the UEFI specification. See the
// uefi-base module for generic UEFI options.

use crate::spec::{base, LinkerFlavor, Lld, Target, TargetOptions};

pub static TARGET: Target = {
    let mut base = base::uefi_msvc::opts();

    base.max_atomic_width = Some(128);
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(LinkerFlavor::Msvc(Lld::No), &["/machine:arm64"])
    });
    base.features = cow!("+v8a");

    Target {
        llvm_target: cow!("aarch64-unknown-windows"),
        pointer_width: 64,
        data_layout: cow!("e-m:w-p:64:64-i32:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: base,
    }
};
