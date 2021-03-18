#[link(wasm_import_module = "ui")]
extern {
    #[link_name = "set_info"] fn _set_info(title: *const u8, content: *const u8);
    #[link_name = "set_info_hex"] fn _set_info_hex(title: *const u8, content: u32);
    #[link_name = "set_bg_color"] fn _set_bg_color(color: u32);
}

pub fn set_info(title: &str, content: &str) {
    unsafe { _set_info(title.as_ptr(), content.as_ptr()) }
}

pub fn set_info_hex(title: &str, content: u32) {
    unsafe { _set_info_hex(title.as_ptr(), content) }
}

pub fn set_bg_color(color: u32) {
    unsafe { _set_bg_color(color) }
}
