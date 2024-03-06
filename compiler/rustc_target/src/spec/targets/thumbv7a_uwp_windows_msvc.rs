use crate::spec::cow;
use crate::spec::{base, PanicStrategy, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        llvm_target: cow!("thumbv7a-pc-windows-msvc"),
        pointer_width: 32,
        data_layout: cow!("e-m:w-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: cow!("arm"),
        options: TargetOptions {
            features: cow!("+vfp3,+neon"),
            max_atomic_width: Some(64),
            // FIXME(jordanrh): use PanicStrategy::Unwind when SEH is
            // implemented for windows/arm in LLVM
            panic_strategy: PanicStrategy::Abort,
            ..base::windows_uwp_msvc::opts()
        },
    }
};
