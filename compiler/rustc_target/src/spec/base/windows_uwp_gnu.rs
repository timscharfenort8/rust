use std::sync::LazyLock;

use crate::spec::{add_link_args, base, Cc, LinkArgs, LinkerFlavor, Lld, TargetOptions};

pub fn opts() -> TargetOptions {
    let base = base::windows_gnu::opts();

    TargetOptions {
        abi: "uwp".into(),
        vendor: "uwp".into(),
        limit_rdylib_exports: false,

        late_link_args: LazyLock::new(|| {
            // FIXME: This should be updated for the exception machinery changes from #67502
            // and inherit from `windows_gnu_base`, at least partially.
            const MINGW_LIBS: &'static [&'static str] = &[
                "-lwinstorecompat",
                "-lruntimeobject",
                "-lsynchronization",
                "-lvcruntime140_app",
                "-lucrt",
                "-lwindowsapp",
                "-lmingwex",
                "-lmingw32",
            ];

            let mut late_link_args =
                TargetOptions::link_args(LinkerFlavor::Gnu(Cc::No, Lld::No), MINGW_LIBS);
            add_link_args(&mut late_link_args, LinkerFlavor::Gnu(Cc::Yes, Lld::No), MINGW_LIBS);
            late_link_args
        }),

        // Reset the flags back to empty until the FIXME above is addressed.
        late_link_args_dynamic: LazyLock::new(LinkArgs::new),
        late_link_args_static: LazyLock::new(LinkArgs::new),

        ..base
    }
}
