use crate::abi::Endian;
use crate::spec::cow;

use crate::spec::{base, SanitizerSet, StackProbeType, Target};

pub fn target() -> Target {
    let mut base = base::linux_musl::opts();
    base.endian = Endian::Big;
    // z10 is the oldest CPU supported by LLVM
    base.cpu = cow!("z10");
    // FIXME: The ABI implementation in cabi_s390x.rs is for now hard-coded to assume the no-vector
    // ABI. Pass the -vector feature string to LLVM to respect this assumption. On LLVM < 16, we
    // also strip v128 from the data_layout below to match the older LLVM's expectation.
    base.features = cow!("-vector");
    base.max_atomic_width = Some(64);
    base.min_global_align = Some(16);
    base.static_position_independent_executables = true;
    base.stack_probes = StackProbeType::Inline;
    base.supported_sanitizers =
        SanitizerSet::ADDRESS | SanitizerSet::LEAK | SanitizerSet::MEMORY | SanitizerSet::THREAD;

    Target {
        llvm_target: cow!("s390x-unknown-linux-musl"),
        pointer_width: 64,
        data_layout: cow!("E-m:e-i1:8:16-i8:8:16-i64:64-f128:64-v128:64-a:8:16-n32:64"),
        arch: cow!("s390x"),
        options: base,
    }
}
