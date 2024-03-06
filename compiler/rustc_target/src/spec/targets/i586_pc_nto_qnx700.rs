use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("i586-pc-unknown"),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i128:128-f64:32:64-f80:32-n8:16:32-S128"
            .into(),
        arch: cow!("x86"),
        options: TargetOptions {
            cpu: cow!("pentium4"),
            max_atomic_width: Some(64),
            pre_link_args: LazyLock::new(|| {
                TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-Vgcc_ntox86_cxx"])
            }),
            env: cow!("nto70"),
            stack_probes: StackProbeType::Inline,
            ..base::nto_qnx::opts()
        },
    }
}
