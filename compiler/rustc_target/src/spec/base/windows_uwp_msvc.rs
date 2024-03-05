use std::sync::LazyLock;

use crate::spec::{base, LinkerFlavor, Lld, TargetOptions};

pub fn opts() -> TargetOptions {
    let mut opts = base::windows_msvc::opts();

    opts.abi = "uwp".into();
    opts.vendor = "uwp".into();
    opts.pre_link_args = LazyLock::new(|| {
        TargetOptions::link_args(LinkerFlavor::Msvc(Lld::No), &["/APPCONTAINER", "mincore.lib"])
    });

    opts
}
