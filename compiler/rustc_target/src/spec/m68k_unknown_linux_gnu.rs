use crate::abi::Endian;
use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::linux_base::opts();
    base.max_atomic_width = Some(32);
    base.endian = Endian::Big;

    Target {
        llvm_target: "m68k-unknown-linux-gnu".to_string(),
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-i8:8:8-i16:16:16-i32:32:32-n8:16:32-a:0:32-S16".to_string(),
        arch: "m68k".to_string(),
        options: base,
    }
}
