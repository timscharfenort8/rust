use crate::spec::base::apple::{ios_llvm_target, opts, Arch};
use crate::spec::cow;

use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    let arch = Arch::Armv7s;
    Target {
        llvm_target: ios_llvm_target(arch).into(),
        pointer_width: 32,
        data_layout: cow!("e-m:o-p:32:32-Fi8-f64:32:64-v64:32:64-v128:32:128-a:0:32-n32-S32"),
        arch: arch.target_arch(),
        options: TargetOptions {
            features: cow!("+v7,+vfp4,+neon"),
            max_atomic_width: Some(64),
            ..opts("ios", arch)
        },
    }
}
