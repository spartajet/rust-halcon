#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use super::*;
    use std::os::raw::c_char;
    use std::str::FromStr;

    #[test]
    fn read_image_test() {
        let mut image: Hobject = unsafe { std::mem::zeroed() };
        let file_name = "../images/1.png";
        let file_name_cstring = CString::new(file_name).unwrap();
        let file_name_c_char = file_name_cstring.as_ptr();
        unsafe {
            let error = read_image(&mut image, file_name_c_char);
            println!("{:?}", error);
        }
    }
}
