use std::sync::LazyLock;

use crate::spec::base::apple::{macos_llvm_target, opts, pre_link_args, Arch};
use crate::spec::{FramePointer, SanitizerSet, Target, TargetOptions};

pub fn target() -> Target {
    const ARCH: Arch = Arch::Arm64;
    const OS: &'static str = "macos";

    let mut base = opts(OS, ARCH);
    base.cpu = "apple-m1".into();
    base.max_atomic_width = Some(128);

    // FIXME: The leak sanitizer currently fails the tests, see #88132.
    base.supported_sanitizers = SanitizerSet::ADDRESS | SanitizerSet::CFI | SanitizerSet::THREAD;

    base.pre_link_args = LazyLock::new(|| pre_link_args(OS, ARCH, ARCH.target_abi()));

    Target {
        // Clang automatically chooses a more specific target based on
        // MACOSX_DEPLOYMENT_TARGET. To enable cross-language LTO to work
        // correctly, we do too.
        llvm_target: macos_llvm_target(ARCH).into(),
        pointer_width: 64,
        data_layout: "e-m:o-i64:64-i128:128-n32:64-S128".into(),
        arch: ARCH.target_arch(),
        options: TargetOptions {
            mcount: "\u{1}mcount".into(),
            frame_pointer: FramePointer::NonLeaf,
            ..base
        },
    }
}
