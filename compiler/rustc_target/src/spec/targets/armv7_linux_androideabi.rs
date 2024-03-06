use crate::spec::cow;
use crate::spec::{base, Cc, LinkerFlavor, Lld, SanitizerSet, Target, TargetOptions};

use std::sync::LazyLock;

// This target if is for the baseline of the Android v7a ABI
// in thumb mode. It's named armv7-* instead of thumbv7-*
// for historical reasons. See the thumbv7neon variant for
// enabling NEON.

// See https://developer.android.com/ndk/guides/abis.html#v7a
// for target ABI requirements.

pub fn target() -> Target {
    let mut base = base::android::opts();
    base.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-march=armv7-a"])
    });
    Target {
        llvm_target: cow!("armv7-none-linux-android"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),
        options: TargetOptions {
            abi: cow!("eabi"),
            features: cow!("+v7,+thumb-mode,+thumb2,+vfp3,-d32,-neon"),
            supported_sanitizers: SanitizerSet::ADDRESS,
            max_atomic_width: Some(64),
            ..base
        },
    }
}
