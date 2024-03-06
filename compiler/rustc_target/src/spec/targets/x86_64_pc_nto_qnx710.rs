use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::{base, Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub static TARGET: Target = Target {
    llvm_target: cow!("x86_64-pc-unknown"),
    pointer_width: 64,
    data_layout: cow!(
        "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
    ),
    arch: cow!("x86_64"),
    options: TargetOptions {
        cpu: cow!("x86-64"),
        plt_by_default: false,
        max_atomic_width: Some(64),
        pre_link_args: LazyLock::new(|| {
            TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-Vgcc_ntox86_64_cxx"])
        }),
        env: cow!("nto71"),
        ..base::nto_qnx::opts()
    },
};
