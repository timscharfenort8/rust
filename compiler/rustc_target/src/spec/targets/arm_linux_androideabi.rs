use crate::spec::cow;
use crate::spec::{base, SanitizerSet, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("arm-linux-androideabi"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),
        options: TargetOptions {
            abi: cow!("eabi"),
            // https://developer.android.com/ndk/guides/abis.html#armeabi
            features: cow!("+strict-align,+v5te"),
            supported_sanitizers: SanitizerSet::ADDRESS,
            max_atomic_width: Some(32),
            ..base::android::opts()
        },
    }
}
