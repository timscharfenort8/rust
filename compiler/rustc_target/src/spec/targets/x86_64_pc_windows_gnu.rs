use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::{base, Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = base::windows_gnu::opts();
    base.cpu = cow!("x86-64");
    base.features = cow!("+cx16,+sse3,+sahf");
    base.plt_by_default = false;
    // Use high-entropy 64 bit address space for ASLR
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(
            LinkerFlavor::Gnu(Cc::No, Lld::No),
            &["-m", "i386pep", "--high-entropy-va"],
        )
    });
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(
            LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            &["-m64", "-Wl,--high-entropy-va"],
        )
    });
    base.max_atomic_width = Some(128);
    base.linker = Some(cow!("x86_64-w64-mingw32-gcc"));

    Target {
        llvm_target: cow!("x86_64-pc-windows-gnu"),
        pointer_width: 64,
        data_layout: cow!(
            "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        arch: cow!("x86_64"),
        options: base,
    }
}
