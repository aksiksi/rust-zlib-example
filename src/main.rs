mod zlib;

fn main() {
    println!("Hello, world!");

    let in_string = &mut String::from("Hello, world!");

    let mut out_arr = vec![0 as u8; in_string.len()];
    let out_ptr = out_arr.as_mut_ptr();

    // Build empty stream struct
    let defstream = zlib::z_stream::default();

    unsafe {
        let in_ptr = in_string.as_bytes_mut().as_mut_ptr();

        // Update with required inputs
        let mut defstream = zlib::z_stream {
            avail_in: in_string.len() as u32,
            next_in: in_ptr,
            avail_out: out_arr.len() as u32,
            next_out: out_ptr,
            ..defstream
        };

        zlib::deflateInit_(&mut defstream,
                           zlib::Z_BEST_COMPRESSION as i32,
                           zlib::ZLIB_VERSION.as_ptr() as *mut i8,
                           std::mem::size_of::<zlib::z_stream>() as i32);
        zlib::deflate(&mut defstream, zlib::Z_FINISH as i32);
        zlib::deflateEnd(&mut defstream);
    }

    println!("{}", in_string);

    for c in out_arr {
        println!("Char: {}", c);
    }
}
