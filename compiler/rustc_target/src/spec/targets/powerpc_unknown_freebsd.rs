use crate::spec::cow;
use std::sync::LazyLock;

use crate::abi::Endian;
use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = base::freebsd::opts();
    // Extra hint to linker that we are generating secure-PLT code.
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(
            LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            &["-m32", "--target=powerpc-unknown-freebsd13.0"],
        )
    });
    base.max_atomic_width = Some(32);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("powerpc-unknown-freebsd13.0"),
        pointer_width: 32,
        data_layout: cow!("E-m:e-p:32:32-Fn32-i64:64-n32"),
        arch: cow!("powerpc"),
        options: TargetOptions {
            endian: Endian::Big,
            features: cow!("+secure-plt"),
            mcount: cow!("_mcount"),
            ..base
        },
    }
}
