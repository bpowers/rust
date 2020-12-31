use super::{LinkerFlavor, PanicStrategy, RelocModel, Target, TargetOptions, TargetResult};

pub fn options() -> TargetOptions {
    // largely copied from WASM
    TargetOptions {
        // is_like_wasm: true,
        // families: vec!["wasm".to_string()],

        dynamic_linking: false,

        executables: true,

        // relatively self-explanatory!
        exe_suffix: ".aaplelf".to_string(),
        eh_frame_header: false,

        max_atomic_width: Some(32),

        panic_strategy: PanicStrategy::Abort,

        singlethread: true,

        // no dynamic linking, no need for default visibility!
        default_hidden_visibility: true,

        relocation_model: RelocModel::Static,

        has_elf_tls: false,

        emit_debug_gdb_scripts: false,

        ..Default::default()
    }
}


pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "m68k-apple-macosclassic".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:e-p:32:32-i8:8:8-i16:16:16-i32:32:32-n8:16:32-a:0:32-S16".to_string(),
        arch: "m68k".to_string(),
        target_os: "apple".to_string(),
        target_env: "macosclassic".to_string(),
        target_vendor: String::new(),
        linker_flavor: LinkerFlavor::Gcc,
        options: options(),
    })
}
