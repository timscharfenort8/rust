use crate::spec::cow;
use std::sync::LazyLock;

use crate::spec::{crt_objects, cvs, Cc, LinkOutputKind, LinkerFlavor, Lld, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: cow!("fuchsia"),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some(cow!("rust-lld")),
        dynamic_linking: true,
        families: cvs!["unix"],
        // This mirrors the linker options provided by clang. We presume lld for
        // now. When using clang as the linker it will supply these options for us,
        // so we only list them for ld/lld.
        //
        // https://github.com/llvm/llvm-project/blob/db9322b2066c55254e7691efeab863f43bfcc084/clang/lib/Driver/ToolChains/Fuchsia.cpp#L31
        pre_link_args: LazyLock::new(|| {
            TargetOptions::link_args(
                LinkerFlavor::Gnu(Cc::No, Lld::No),
                &[
                    "--build-id",
                    "--hash-style=gnu",
                    "-z",
                    "max-page-size=4096",
                    "-z",
                    "now",
                    "-z",
                    "rodynamic",
                    "-z",
                    "separate-loadable-segments",
                    "--pack-dyn-relocs=relr",
                ],
            )
        }),
        pre_link_objects: LazyLock::new(|| {
            crt_objects::new(&[
                (LinkOutputKind::DynamicNoPicExe, &["Scrt1.o"]),
                (LinkOutputKind::DynamicPicExe, &["Scrt1.o"]),
                (LinkOutputKind::StaticNoPicExe, &["Scrt1.o"]),
                (LinkOutputKind::StaticPicExe, &["Scrt1.o"]),
            ])
        }),
        position_independent_executables: true,
        has_thread_local: true,
        ..TargetOptions::default()
    }
}
