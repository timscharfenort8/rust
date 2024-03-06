use crate::spec::cow;
use crate::spec::{base, Cc, LinkerFlavor, Target};

pub fn target() -> Target {
    let mut base = base::linux_musl::opts();
    base.cpu = cow!("hexagonv60");
    base.max_atomic_width = Some(32);
    // FIXME: HVX length defaults are per-CPU
    base.features = cow!("-small-data,+hvx-length128b");

    base.crt_static_default = false;
    base.has_rpath = true;
    base.linker_flavor = LinkerFlavor::Unix(Cc::Yes);

    base.c_enum_min_bits = Some(8);

    Target {
        llvm_target: cow!("hexagon-unknown-linux-musl"),
        pointer_width: 32,
        data_layout: concat!(
            "e-m:e-p:32:32:32-a:0-n16:32-i64:64:64-i32:32",
            ":32-i16:16:16-i1:8:8-f32:32:32-f64:64:64-v32",
            ":32:32-v64:64:64-v512:512:512-v1024:1024:1024-v2048",
            ":2048:2048"
        )
        .into(),
        arch: cow!("hexagon"),
        options: base,
    }
}
