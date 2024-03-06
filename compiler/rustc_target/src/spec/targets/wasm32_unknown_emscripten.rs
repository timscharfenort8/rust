use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::{
    base, cvs, LinkArgs, LinkerFlavor, PanicStrategy, RelocModel, Target, TargetOptions,
};

pub fn target() -> Target {
    let opts = TargetOptions {
        os: cow!("emscripten"),
        linker_flavor: LinkerFlavor::EmCc,
        // emcc emits two files - a .js file to instantiate the wasm and supply platform
        // functionality, and a .wasm file.
        exe_suffix: cow!(".js"),
        linker: None,
        // Reset flags for non-Em flavors back to empty to satisfy sanity checking tests.
        pre_link_args: LazyLock::new(LinkArgs::new),
        post_link_args: LazyLock::new(|| {
            TargetOptions::link_args(LinkerFlavor::EmCc, &["-sABORTING_MALLOC=0"])
        }),
        relocation_model: RelocModel::Pic,
        panic_strategy: PanicStrategy::Unwind,
        no_default_libraries: false,
        families: cvs!["unix", "wasm"],
        ..base::wasm::options()
    };
    Target {
        llvm_target: cow!("wasm32-unknown-emscripten"),
        pointer_width: 32,
        data_layout: cow!("e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-f128:64-n32:64-S128-ni:1:10:20"),
        arch: cow!("wasm32"),
        options: opts,
    }
}
