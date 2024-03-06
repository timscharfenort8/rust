use crate::spec::base::apple::{opts, Arch};
use crate::spec::cow;

use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    let arch = Arch::Armv7k;
    Target {
        llvm_target: cow!("armv7k-apple-watchos"),
        pointer_width: 32,
        data_layout: cow!("e-m:o-p:32:32-Fi8-i64:64-a:0:32-n32-S128"),
        arch: arch.target_arch(),
        options: TargetOptions {
            features: cow!("+v7,+vfp4,+neon"),
            max_atomic_width: Some(64),
            dynamic_linking: false,
            position_independent_executables: true,
            ..opts("watchos", arch)
        },
    }
}
