use crate::spec::cow;
use crate::spec::{cvs, Cc, LinkerFlavor, PanicStrategy, RelocModel, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: cow!("l4re"),
        env: cow!("uclibc"),
        linker_flavor: LinkerFlavor::Unix(Cc::No),
        panic_strategy: PanicStrategy::Abort,
        linker: Some(cow!("l4-bender")),
        families: cvs!["unix"],
        relocation_model: RelocModel::Static,
        ..TargetOptions::default()
    }
}
