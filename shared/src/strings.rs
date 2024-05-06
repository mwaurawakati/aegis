use crate::log::error;


pub fn crash<S: AsRef<str>>(a: S, _b: i32) -> ! {
    error!("{}", a.as_ref());
    panic!();
}
