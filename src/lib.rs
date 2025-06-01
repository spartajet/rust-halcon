#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{ffi::CString, os::raw::c_char};

include!("bindings.rs");

fn str_to_c_char(str: &str) -> *const c_char {
    let c_string = CString::new(str).expect("CString::new failed");
    c_string.as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::CString, os::raw::c_char};

    #[test]
    fn read_image_test() {
        let mut image: Hobject = std::ptr::null_mut();
        let file_name = "images/1.png";
        // let file_path = PathBuf::from(file_name);
        // if !file_path.exists() {
        //     panic!("File does not exist: {}", file_name);
        // }
        let file_name_cstring = CString::new(file_name).unwrap();
        let file_name_c_char = file_name_cstring.as_ptr();
        let error = unsafe { read_image(&mut image, file_name_c_char) };
        assert_eq!(error, H_MSG_OK, "Failed to read image: {}", error);
        let mut width: Hlong = 0;
        let mut height: Hlong = 0;
        let error = unsafe { get_image_size(image, &mut width, &mut height) };
        println!("width: {}, height: {}", width, height);
        assert_eq!(error, H_MSG_OK, "Failed to get image size: {}", error);
        assert_ne!(width, 0);
        assert_ne!(height, 0);
        let format_cstring = CString::new("png").unwrap();
        let format_c_char = format_cstring.as_ptr();
        let save_file_name = CString::new("output.png").unwrap();
        let save_file_name_c_char = save_file_name.as_ptr();
        let error = unsafe { write_image(image, format_c_char, 0, save_file_name_c_char) };
        assert_eq!(error, H_MSG_OK, "Failed to write image: {}", error);
    }
}
