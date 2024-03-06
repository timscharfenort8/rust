use crate::spec::cow;
use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::i686_unknown_linux_musl::target();
    base.cpu = cow!("pentium");
    base.llvm_target = cow!("i586-unknown-linux-musl");
    base
}
