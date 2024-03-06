use crate::spec::cow;
use crate::spec::{base, SanitizerSet, StackProbeType, Target, TargetOptions};

// See https://developer.android.com/ndk/guides/abis.html#x86
// for target ABI requirements.

pub static TARGET: Target = {
    let mut base = base::android::opts();

    base.max_atomic_width = Some(64);

    // https://developer.android.com/ndk/guides/abis.html#x86
    base.cpu = cow!("pentiumpro");
    base.features = cow!("+mmx,+sse,+sse2,+sse3,+ssse3");
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: cow!("i686-linux-android"),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i128:128-f64:32:64-f80:32-n8:16:32-S128"
            .into(),
        arch: cow!("x86"),
        options: TargetOptions { supported_sanitizers: SanitizerSet::ADDRESS, ..base },
    }
};
