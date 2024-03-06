use crate::spec::cow;
use crate::spec::{cvs, Cc, LinkerFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: cow!("solaris"),
        dynamic_linking: true,
        has_rpath: true,
        families: cvs!["unix"],
        is_like_solaris: true,
        linker_flavor: LinkerFlavor::Unix(Cc::Yes),
        limit_rdylib_exports: false, // Linker doesn't support this
        eh_frame_header: false,

        ..TargetOptions::default()
    }
}
