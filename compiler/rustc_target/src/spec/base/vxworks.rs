use crate::spec::cow;
use crate::spec::{cvs, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: cow!("vxworks"),
        env: cow!("gnu"),
        vendor: cow!("wrs"),
        linker: Some(cow!("wr-c++")),
        exe_suffix: cow!(".vxe"),
        dynamic_linking: true,
        families: cvs!["unix"],
        has_rpath: true,
        has_thread_local: true,
        crt_static_default: true,
        crt_static_respected: true,
        crt_static_allows_dylibs: true,
        // VxWorks needs to implement this to support profiling
        mcount: cow!("_mcount"),
        ..TargetOptions::default()
    }
}
