use crate::spec::cow;
use crate::spec::Target;

use crate::{abi::Endian, spec::base};

pub fn target() -> Target {
    Target {
        llvm_target: cow!("bpfeb"),
        data_layout: cow!("E-m:e-p:64:64-i64:64-i128:128-n32:64-S128"),
        pointer_width: 64,
        arch: cow!("bpf"),
        options: base::bpf::opts(Endian::Big),
    }
}
