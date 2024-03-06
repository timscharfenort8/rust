use crate::spec::cow;
use crate::spec::{FramePointer, TargetOptions};

pub fn opts(kernel: &str) -> TargetOptions {
    TargetOptions {
        os: format!("solid_{kernel}").into(),
        vendor: cow!("kmc"),
        executables: false,
        frame_pointer: FramePointer::NonLeaf,
        has_thread_local: true,
        ..TargetOptions::default()
    }
}
