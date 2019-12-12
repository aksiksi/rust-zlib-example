mod zlib;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_then_decompress() {
        let mut in_string = String::from("Hello, world!");
        let in_ptr = in_string.as_mut_str().as_mut_ptr();

        let mut out_arr = vec![0 as u8; 64];
        let out_ptr = out_arr.as_mut_ptr();

        let mut final_arr = vec![0 as u8; in_string.len()];
        let final_ptr = final_arr.as_mut_ptr();

        // Build empty stream struct
        let defstream = zlib::z_stream::default();

        // Update with required inputs
        let mut defstream = zlib::z_stream {
            avail_in: in_string.len() as u32,
            next_in: in_ptr,
            avail_out: out_arr.len() as u32,
            next_out: out_ptr,
            ..defstream
        };

        // Compress
        unsafe {
            zlib::deflateInit_(&mut defstream,
                               zlib::Z_BEST_COMPRESSION as i32,
                               zlib::ZLIB_VERSION.as_ptr() as *mut i8,
                               std::mem::size_of::<zlib::z_stream>() as i32);
            zlib::deflate(&mut defstream, zlib::Z_FINISH as i32);
            zlib::deflateEnd(&mut defstream);
        }

        // Input is now the compressed output from above
        let mut defstream = zlib::z_stream {
            avail_in: out_arr.len() as u32,
            next_in: out_ptr,
            avail_out: final_arr.len() as u32,
            next_out: final_ptr,
            ..defstream
        };

        // Decompress
        unsafe {
            zlib::inflateInit_(&mut defstream,
                               zlib::ZLIB_VERSION.as_ptr() as *mut i8,
                               std::mem::size_of::<zlib::z_stream>() as i32);
            zlib::inflate(&mut defstream, zlib::Z_FINISH as i32);
            zlib::inflateEnd(&mut defstream);
        }

        assert_eq!(&in_string, &std::str::from_utf8(&final_arr).unwrap());
    }
}
