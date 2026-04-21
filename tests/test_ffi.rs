#![deny(improper_ctypes, improper_ctypes_definitions)]

use wallee::wallee;

#[unsafe(no_mangle)]
pub extern "C" fn wallee1(err: wallee::Error) {
    println!("{err:?}");
}

#[unsafe(no_mangle)]
pub extern "C" fn wallee2(err: &mut Option<wallee::Error>) {
    *err = Some(wallee!("ffi error"));
}

#[unsafe(no_mangle)]
pub extern "C" fn wallee3() -> Option<wallee::Error> {
    Some(wallee!("ffi error"))
}
