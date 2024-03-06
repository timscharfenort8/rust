use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};
use std::sync::LazyLock;

pub fn target() -> Target {
    let mut base = base::vxworks::opts();
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m32", "--secure-plt"])
    });
    base.max_atomic_width = Some(32);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("powerpc-unknown-linux-gnu"),
        pointer_width: 32,
        data_layout: cow!("E-m:e-p:32:32-Fn32-i64:64-n32"),
        arch: cow!("powerpc"),
        options: TargetOptions { endian: Endian::Big, features: cow!("+secure-plt"), ..base },
    }
}
