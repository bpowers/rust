use super::{PanicStrategy, RelocModel, Target, TargetOptions};
use crate::abi::Endian;

pub fn options() -> TargetOptions {
    // largely copied from WASM
    TargetOptions {
        eh_frame_header: false,
        endian: Endian::Big,
        dynamic_linking: false,
        executables: true,
        relocation_model: RelocModel::Static,
        max_atomic_width: Some(32),
        disable_redzone: true,
        eliminate_frame_pointer: false,
        no_default_libraries: false,
        has_elf_tls: false,
        panic_strategy: PanicStrategy::Abort,
        default_hidden_visibility: true,
        singlethread: true,
        ..Default::default()
    }
}

pub fn target() -> Target {
    Target {
        llvm_target: "m68k-apple-macosclassic".to_string(),
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-i8:8:8-i16:16:16-i32:32:32-n8:16:32-a:0:32-S16".to_string(),
        arch: "m68k".to_string(),
        options: options(),
    }
}
