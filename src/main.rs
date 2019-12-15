mod zlib;

fn compress(input: &mut str) -> Vec<u8> {
    // TODO: We need to setup a correct array size
    // Use the outputs of z_stream to figure out
    // if deflating is complete.
    let mut out_arr: Vec<u8> = vec![0; 64];

    // Build empty stream struct
    let defstream = zlib::z_stream::default();

    // Update with required inputs
    let mut defstream = zlib::z_stream {
        avail_in: input.len() as u32,
        next_in: input.as_mut_ptr(),
        avail_out: out_arr.len() as u32,
        next_out: out_arr.as_mut_ptr(),
        ..defstream
    };

    unsafe {
        zlib::deflateInit_(&mut defstream,
                           zlib::Z_BEST_COMPRESSION as i32,
                           zlib::ZLIB_VERSION.as_ptr() as *mut i8,
                           std::mem::size_of::<zlib::z_stream>() as i32);
        zlib::deflate(&mut defstream, zlib::Z_FINISH as i32);
        zlib::deflateEnd(&mut defstream);
    }

    return out_arr;
}

fn decompress(input: &mut Vec<u8>, len: usize) -> String {
    // TODO: Same note as above - inflate can be chunked
    let mut out_arr = vec![0 as u8; len];

    // Build empty stream struct
    let defstream = zlib::z_stream::default();

    // Update with required inputs
    let mut defstream = zlib::z_stream {
        avail_in: input.len() as u32,
        next_in: input.as_mut_ptr(),
        avail_out: out_arr.len() as u32,
        next_out: out_arr.as_mut_ptr(),
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

    return String::from_utf8(out_arr).unwrap();
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_then_decompress() {
        let mut in_string = String::from("Hello, world!");
        let mut compressed = compress(&mut in_string[..]);
        let decompressed = decompress(&mut compressed, in_string.len());

        assert_eq!(in_string, decompressed);
    }
}
