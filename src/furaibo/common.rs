#[macro_export]
macro_rules! f_str {
    ($s:tt) => { concat!($s, '\0') }
}