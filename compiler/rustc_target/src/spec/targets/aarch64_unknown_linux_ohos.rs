use crate::spec::cow;
use crate::spec::SanitizerSet;

use crate::spec::{base, StackProbeType, Target, TargetOptions};

pub static TARGET: Target = {
    let mut base = base::linux_ohos::opts();
    base.max_atomic_width = Some(128);

    Target {
        // LLVM 15 doesn't support OpenHarmony yet, use a linux target instead.
        llvm_target: cow!("aarch64-unknown-linux-musl"),
        pointer_width: 64,
        data_layout: cow!("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: TargetOptions {
            features: cow!("+reserve-x18"),
            mcount: cow!("\u{1}_mcount"),
            stack_probes: StackProbeType::Inline,
            supported_sanitizers: SanitizerSet::ADDRESS
                | SanitizerSet::CFI
                | SanitizerSet::LEAK
                | SanitizerSet::MEMORY
                | SanitizerSet::MEMTAG
                | SanitizerSet::THREAD
                | SanitizerSet::HWADDRESS,
            ..base
        },
    }
};
