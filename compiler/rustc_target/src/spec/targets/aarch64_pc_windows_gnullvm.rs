use crate::spec::cow;
use crate::spec::{base, Target};

pub static TARGET: Target = {
    let mut base = base::windows_gnullvm::opts();
    base.max_atomic_width = Some(128);
    base.features = cow!("+v8a,+neon,+fp-armv8");
    base.linker = Some(cow!("aarch64-w64-mingw32-clang"));

    Target {
        llvm_target: cow!("aarch64-pc-windows-gnu"),
        pointer_width: 64,
        data_layout: cow!("e-m:w-p:64:64-i32:32-i64:64-i128:128-n32:64-S128"),
        arch: cow!("aarch64"),
        options: base,
    }
};
