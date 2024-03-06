use crate::spec::cow;
use std::borrow::Cow;

use std::sync::LazyLock;

use crate::spec::{cvs, Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub static TARGET: Target = {
    const EXPORT_SYMBOLS: &[&str] = &[
        "sgx_entry",
        "HEAP_BASE",
        "HEAP_SIZE",
        "RELA",
        "RELACOUNT",
        "ENCLAVE_SIZE",
        "CFGDATA_BASE",
        "DEBUG",
        "EH_FRM_HDR_OFFSET",
        "EH_FRM_HDR_LEN",
        "EH_FRM_OFFSET",
        "EH_FRM_LEN",
        "TEXT_BASE",
        "TEXT_SIZE",
    ];
    let opts = TargetOptions {
        os: cow!("unknown"),
        env: cow!("sgx"),
        vendor: cow!("fortanix"),
        abi: cow!("fortanix"),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some(cow!("rust-lld")),
        max_atomic_width: Some(64),
        cpu: cow!("x86-64"),
        plt_by_default: false,
        features: cow!("+rdrnd,+rdseed,+lvi-cfi,+lvi-load-hardening"),
        llvm_args: cvs!["--x86-experimental-lvi-inline-asm-hardening"],
        position_independent_executables: true,
        pre_link_args: LazyLock::new(|| {
            TargetOptions::link_args(
                LinkerFlavor::Gnu(Cc::No, Lld::No),
                &[
                    "-e",
                    "elf_entry",
                    "-Bstatic",
                    "--gc-sections",
                    "-z",
                    "text",
                    "-z",
                    "norelro",
                    "--no-undefined",
                    "--error-unresolved-symbols",
                    "--no-undefined-version",
                    "-Bsymbolic",
                    "--export-dynamic",
                    // The following symbols are needed by libunwind, which is linked after
                    // libstd. Make sure they're included in the link.
                    "-u",
                    "__rust_abort",
                    "-u",
                    "__rust_c_alloc",
                    "-u",
                    "__rust_c_dealloc",
                    "-u",
                    "__rust_print_err",
                    "-u",
                    "__rust_rwlock_rdlock",
                    "-u",
                    "__rust_rwlock_unlock",
                    "-u",
                    "__rust_rwlock_wrlock",
                ],
            )
        }),
        override_export_symbols: Some(EXPORT_SYMBOLS.iter().cloned().map(Cow::from).collect()),
        relax_elf_relocations: true,
        ..TargetOptions::default()
    };
    Target {
        llvm_target: cow!("x86_64-elf"),
        pointer_width: 64,
        data_layout: cow!(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        arch: cow!("x86_64"),
        options: opts,
    }
};
