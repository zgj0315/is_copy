//! # Usage
//!
//! Add this to your *Cargo.toml*:
//! ```toml
//! [dependencies]
//! is_copy = "0.1"
//! ```
//!
//! # Examples
//! Create and determin file using is_copy:
//! ```
//! use std::{
//!     fs::{self, copy, File},
//!     io::Write,
//!     path::{Path, PathBuf},
//! };
//! use is_copy::is_file_copy;
//!
//! let path_data = Path::new("./data");
//! if path_data.exists() {
//!     fs::remove_dir_all(path_data).unwrap();
//! }
//! fs::create_dir_all(path_data).unwrap();
//! let path_a = path_data.join("file_a.txt");
//! let path_a_copy = path_data.join("file_a_copy.txt");
//! let path_b = path_data.join("file_b.txt");
//! let mut file_a = File::create(&path_a).unwrap();
//! let mut file_b = File::create(&path_b).unwrap();
//! file_a.write_all(b"this is file a").unwrap();
//! file_b.write_all(b"this is file b").unwrap();
//! for i in 0..1_000 {
//!     let line = format!("this is line {}\n", i);
//!     file_a.write_all(line.as_bytes()).unwrap();
//!     file_b.write_all(line.as_bytes()).unwrap();
//! }
//! copy(&path_a, &path_a_copy).unwrap();
//! assert!(is_file_copy(&path_a, &path_a_copy));
//! assert!(!is_file_copy(&path_a, &path_b));
//! ```

use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub fn is_file_copy(path_a: &Path, path_b: &Path) -> bool {
    log::trace!("path_a: {:?}, path_b: {:?}", path_a, path_b);
    if path_a == path_b {
        log::trace!("same file");
        return true;
    }
    log::trace!("not same file");
    let input_size = fs::metadata(path_a).unwrap().len();
    let output_size = fs::metadata(path_b).unwrap().len();
    if input_size != output_size {
        log::trace!("diff size, {:?} and {:?}", input_size, output_size);
        return false;
    }
    log::trace!("same size, {}", input_size);
    let mut file_a = File::open(path_a).unwrap();
    let mut file_b = File::open(path_b).unwrap();
    let mut buf_a = Vec::with_capacity(input_size.try_into().unwrap());
    let mut buf_b = Vec::with_capacity(output_size.try_into().unwrap());
    file_a.read_to_end(&mut buf_a).unwrap();
    file_b.read_to_end(&mut buf_b).unwrap();
    if buf_a.eq(&buf_b) {
        log::trace!("same content");
        return true;
    } else {
        log::trace!("diff content");
        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, copy, File},
        io::Write,
        path::{Path, PathBuf},
    };

    use dev_util::log::log_init;

    use crate::is_file_copy;

    fn init_dir() -> PathBuf {
        let path_dir = Path::new("./data");
        if path_dir.exists() {
            fs::remove_dir_all(path_dir).unwrap();
        }
        fs::create_dir_all(path_dir).unwrap();
        path_dir.to_path_buf()
    }

    fn init_txt(path_dir: &PathBuf) -> (PathBuf, PathBuf, PathBuf) {
        let path_a = path_dir.join("file_a.txt");
        let path_a_copy = path_dir.join("file_a_copy.txt");
        let path_b = path_dir.join("file_b.txt");
        let mut file_a = File::create(&path_a).unwrap();
        let mut file_b = File::create(&path_b).unwrap();
        file_a.write_all(b"this is file a").unwrap();
        file_b.write_all(b"this is file b").unwrap();
        for i in 0..1_000 {
            let line = format!("this is line {}\n", i);
            file_a.write_all(line.as_bytes()).unwrap();
            file_b.write_all(line.as_bytes()).unwrap();
        }
        copy(&path_a, &path_a_copy).unwrap();
        (path_a, path_a_copy, path_b)
    }

    fn init_png(path_dir: &PathBuf) -> (PathBuf, PathBuf, PathBuf) {
        let path_a = path_dir.join("file_a.png");
        let path_a_copy = path_dir.join("file_a_copy.png");
        let path_b = path_dir.join("file_b.png");
        let mut imgbuf = image::ImageBuffer::new(256, 256);
        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            let r: u8 = 229;
            let g: u8 = 229;
            let b: u8 = 229;
            let a: u8 = 229;
            *pixel = image::Rgba([r, g, b, a]);
        }
        imgbuf.save(&path_a).unwrap();

        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            let r: u8 = 29;
            let g: u8 = 29;
            let b: u8 = 29;
            let a: u8 = 29;
            *pixel = image::Rgba([r, g, b, a]);
        }
        imgbuf.save(&path_b).unwrap();
        copy(&path_a, &path_a_copy).unwrap();
        (path_a, path_a_copy, path_b)
    }

    // cargo test tests::test_is_file_copy
    #[test]
    fn test_is_file_copy() {
        log_init();
        let path_dir = init_dir();
        let (path_a, path_a_copy, path_b) = init_txt(&path_dir);
        assert!(is_file_copy(&path_a, &path_a_copy));
        assert!(!is_file_copy(&path_a, &path_b));
        let (path_a, path_a_copy, path_b) = init_png(&path_dir);
        assert!(is_file_copy(&path_a, &path_a_copy));
        assert!(!is_file_copy(&path_a, &path_b));
    }
}
