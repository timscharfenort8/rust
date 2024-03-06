use crate::spec::cow;
use crate::spec::{cvs, PanicStrategy, RelocModel, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: cow!("linux"),
        env: cow!("musl"),
        vendor: cow!("unikraft"),
        linker: Some(cow!("kraftld")),
        relocation_model: RelocModel::Static,
        families: cvs!["unix"],
        has_thread_local: true,
        panic_strategy: PanicStrategy::Abort,
        ..TargetOptions::default()
    }
}
