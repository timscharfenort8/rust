use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, StackProbeType, Target};

pub static TARGET: Target = {
    let mut base = base::openbsd::opts();
    base.endian = Endian::Big;
    base.max_atomic_width = Some(32);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("powerpc-unknown-openbsd"),
        pointer_width: 32,
        data_layout: cow!("E-m:e-p:32:32-Fn32-i64:64-n32"),
        arch: cow!("powerpc"),
        options: base,
    }
};
