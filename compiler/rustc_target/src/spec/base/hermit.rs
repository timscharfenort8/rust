use crate::spec::cow;
use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: cow!("hermit"),
        linker: Some(cow!("rust-lld")),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        tls_model: TlsModel::InitialExec,
        position_independent_executables: true,
        static_position_independent_executables: true,
        has_thread_local: true,
        panic_strategy: PanicStrategy::Abort,
        ..TargetOptions::default()
    }
}
