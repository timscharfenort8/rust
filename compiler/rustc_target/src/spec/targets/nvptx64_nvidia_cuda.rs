use crate::spec::cow;
use crate::spec::{LinkerFlavor, MergeFunctions, PanicStrategy, Target, TargetOptions};

pub static TARGET: Target = {
    Target {
        arch: cow!("nvptx64"),
        data_layout: cow!("e-i64:64-i128:128-v16:16-v32:32-n16:32:64"),
        llvm_target: cow!("nvptx64-nvidia-cuda"),
        pointer_width: 64,

        options: TargetOptions {
            os: cow!("cuda"),
            vendor: cow!("nvidia"),
            linker_flavor: LinkerFlavor::Ptx,
            // The linker can be installed from `crates.io`.
            linker: Some(cow!("rust-ptx-linker")),

            // With `ptx-linker` approach, it can be later overridden via link flags.
            cpu: cow!("sm_30"),

            // FIXME: create tests for the atomics.
            max_atomic_width: Some(64),

            // Unwinding on CUDA is neither feasible nor useful.
            panic_strategy: PanicStrategy::Abort,

            // Needed to use `dylib` and `bin` crate types and the linker.
            dynamic_linking: true,

            // Avoid using dylib because it contain metadata not supported
            // by LLVM NVPTX backend.
            only_cdylib: true,

            // Let the `ptx-linker` to handle LLVM lowering into MC / assembly.
            obj_is_bitcode: true,

            // Convenient and predicable naming scheme.
            dll_prefix: cow!(""),
            dll_suffix: cow!(".ptx"),
            exe_suffix: cow!(".ptx"),

            // Disable MergeFunctions LLVM optimisation pass because it can
            // produce kernel functions that call other kernel functions.
            // This behavior is not supported by PTX ISA.
            merge_functions: MergeFunctions::Disabled,

            // The LLVM backend does not support stack canaries for this target
            supports_stack_protector: false,

            ..TargetOptions::default()
        },
    }
};
