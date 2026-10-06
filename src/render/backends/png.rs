use crate::render::argb::Argb;
use crate::render::image::{ImageBackend, Pixels};

pub struct Png;

impl ImageBackend for Png {
    fn parse_rgb(path: &str) -> std::io::Result<(Vec<Argb>, Pixels, Pixels)> {
        let mut decoder = png::Decoder::new(std::io::BufReader::new(std::fs::File::open(path)?));
        decoder.set_transformations(png::Transformations::EXPAND);
        let mut reader = decoder
            .read_info()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut buf = vec![
            Argb::default();
            reader
                .output_buffer_size()
                .ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Couldn't get output buffer size"
                ))?
        ];
        // This is safe because argb is aligned, repr c and made up of u8 (probably not safe ngl)
        let info = reader.next_frame(unsafe {
            core::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len())
        })?;
        let bytes = &mut buf[..info.buffer_size()];

        // for chunk in bytes.into_iter() {
        // for chunk in bytes.chunks_exact_mut(4) {
        // let ptr = chunk.as_mut_ptr() as *mut u32;
        // This is probably safe becuase it is in chunks of 4 u8's
        // We combine them into a u32 to efficiently bitshift
        // Writing to that u32 should be safe
        // unsafe {
        // *ptr = (chunk[0] as u32).rotate_right(8);
        // }
        // chunk.rotate_right(8);
        // }
        Ok((buf, info.width, info.height))
    }
}

#[test]
fn rotate_test() {
    let buf: &mut [u8] = &mut [1, 0, 0, 0];
    let ptr = buf.as_mut_ptr() as *mut u32;
    unsafe {
        *ptr = (buf[0] as u32).rotate_right(8);
    }
    let check: &[u8] = &[0, 0, 0, 1];
    let checkptr = check.as_ptr() as *mut u32;
    unsafe {
        assert_eq!(*ptr, *checkptr);
    }
}

#[test]
fn png_img_test() {
    let mut rgb: Vec<Argb> = Png::parse_rgb("qoi_test_images/qoi_logo.png").unwrap().0; // Relies on the filesystem, bad test, I know
    rgb.shrink_to_fit(); // Vec doesn't need to grow anymore
    // println!("{:?}", rgb);
}
