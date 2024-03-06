use crate::spec::cow;
use crate::spec::{
    base, Cc, LinkerFlavor, Lld, SanitizerSet, StackProbeType, Target, TargetOptions,
};
use std::sync::LazyLock;

pub fn target() -> Target {
    let mut base = base::android::opts();
    base.cpu = cow!("x86-64");
    base.plt_by_default = false;
    // https://developer.android.com/ndk/guides/abis.html#86-64
    base.features = cow!("+mmx,+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt");
    base.max_atomic_width = Some(64);
    base.pre_link_args =
        LazyLock::new(|| TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]));
    base.stack_probes = StackProbeType::Inline;
    base.supports_xray = true;

    Target {
        llvm_target: cow!("x86_64-linux-android"),
        pointer_width: 64,
        data_layout: cow!(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        arch: cow!("x86_64"),
        options: TargetOptions { supported_sanitizers: SanitizerSet::ADDRESS, ..base },
    }
}
