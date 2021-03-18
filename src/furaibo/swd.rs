#[link(wasm_import_module = "swd")]
extern {
    #[link_name = "read_multi_32"] fn _read_multi_32(addr: u32, out: *mut u32, count: u32) -> i32;
    #[link_name = "read_one_32"] fn _read_one_32(addr: u32, out: *mut u32) -> i32;
    #[link_name = "read_multi_8"] fn _read_multi_8(addr: u32, out: *mut u8, len: u32) -> i32;
    #[link_name = "read_one_8"] fn _read_one_8(addr: u32, out: *mut u8) -> i32;
    #[link_name = "write_multi_32"] fn _write_multi_32(addr: u32, buf: *const u8, len: u32) -> i32;
    #[link_name = "write_one_32"] fn _write_one_32(addr: u32, val: u32) -> i32;
    #[link_name = "write_multi_8"] fn _write_multi_8(addr: u32, buf: *const u8, len: u32) -> i32;
    #[link_name = "write_one_8"] fn _write_one_8(addr: u32, val: u8) -> i32;
    #[link_name = "read_idcode"] fn _read_idcode(val: *mut u8) -> i32;
    #[link_name = "force_reset_session"] fn _force_reset_session() -> i32;
    #[link_name = "reset"] fn _reset() -> i32;
    #[link_name = "halt"] fn _halt() -> i32;
    #[link_name = "reset_halt"] fn _reset_halt() -> i32;
    #[link_name = "is_halted"] fn _is_halted() -> i32;
}

pub fn read_multi_32(addr: u32, out: &mut [u32]) -> i32 {
    unsafe { _read_multi_32(addr, out.as_mut_ptr(), out.len() as u32) }
}

pub fn read_multi_8(addr: u32, out: &mut [u8]) -> i32 {
    unsafe { _read_multi_8(addr, out.as_mut_ptr(), out.len() as u32) }
}

pub fn read_one_32(addr: u32, out: &mut u32) -> i32 {
    unsafe { _read_one_32(addr, out) }
}

pub fn read_one_8(addr: u32, out: &mut u8) -> i32 {
    unsafe { _read_one_8(addr, out) }
}

pub fn write_multi_32(addr: u32, buf: &[u8]) -> i32 {
    unsafe { _write_multi_32(addr, buf.as_ptr(), buf.len() as u32) }
}

pub fn write_multi_8(addr: u32, buf: &[u8]) -> i32 {
    unsafe { _write_multi_8(addr, buf.as_ptr(), buf.len() as u32) }
}

pub fn write_one_32(addr: u32, val: u32) -> i32 {
    unsafe { _write_one_32(addr, val) }
}

pub fn write_one_8(addr: u32, val: u8) -> i32 {
    unsafe { _write_one_8(addr, val) }
}

pub fn reset() -> i32 {
    unsafe { _reset() }
}

pub fn halt() -> i32 {
    unsafe { _halt() }
}

pub fn reset_halt() -> i32 {
    unsafe { _reset_halt() }
}

pub fn force_reset_session() -> i32 {
    unsafe { _force_reset_session() }
}

pub fn is_halted() -> bool {
    unsafe { _is_halted() != 0 }
}