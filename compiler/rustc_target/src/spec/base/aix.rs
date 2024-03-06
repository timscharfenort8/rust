use crate::spec::cow;
use std::sync::LazyLock;

use crate::abi::Endian;
use crate::spec::{crt_objects, cvs, Cc, CodeModel, LinkOutputKind, LinkerFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        abi: cow!("vec-extabi"),
        code_model: Some(CodeModel::Small),
        cpu: cow!("pwr7"),
        os: cow!("aix"),
        vendor: cow!("ibm"),
        dynamic_linking: true,
        endian: Endian::Big,
        executables: true,
        archive_format: cow!("aix_big"),
        families: cvs!["unix"],
        has_rpath: false,
        has_thread_local: true,
        crt_static_respected: true,
        linker_flavor: LinkerFlavor::Unix(Cc::No),
        linker: Some(cow!("ld")),
        eh_frame_header: false,
        is_like_aix: true,
        default_dwarf_version: 3,
        function_sections: true,
        pre_link_objects: LazyLock::new(|| {
            crt_objects::new(&[
                (LinkOutputKind::DynamicNoPicExe, &["/usr/lib/crt0_64.o", "/usr/lib/crti_64.o"]),
                (LinkOutputKind::DynamicPicExe, &["/usr/lib/crt0_64.o", "/usr/lib/crti_64.o"]),
            ])
        }),
        dll_suffix: cow!(".a"),
        ..TargetOptions::default()
    }
}
