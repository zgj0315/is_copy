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
//! for i in 0..1000_000 {
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

use md5::{Digest, Md5};

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
    let mut file = File::open(path_a).unwrap();
    let mut buf = Vec::with_capacity(input_size.try_into().unwrap());
    file.read_to_end(&mut buf).unwrap();
    let input_md5 = format!("{:X}", Md5::digest(buf));
    let mut file = File::open(path_b).unwrap();
    let mut buf = Vec::with_capacity(output_size.try_into().unwrap());
    file.read_to_end(&mut buf).unwrap();
    let output_md5 = format!("{:X}", Md5::digest(buf));
    if input_md5 == output_md5 {
        log::trace!("same md5, {}", input_md5);
        return true;
    } else {
        log::trace!("diff md5, {} and {}", input_md5, output_md5);
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

    use chrono::Local;
    use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

    use crate::is_file_copy;

    fn init_data() -> (PathBuf, PathBuf, PathBuf) {
        struct LocalTimer;
        impl FormatTime for LocalTimer {
            fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
                write!(w, "{}", Local::now().format("%F %T%.3f"))
            }
        }
        let format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_timer(LocalTimer);
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_writer(std::io::stdout)
            .with_ansi(true)
            .event_format(format)
            .init();
        let path_data = Path::new("./data");
        if path_data.exists() {
            fs::remove_dir_all(path_data).unwrap();
        }
        fs::create_dir_all(path_data).unwrap();
        let path_a = path_data.join("file_a.txt");
        let path_a_copy = path_data.join("file_a_copy.txt");
        let path_b = path_data.join("file_b.txt");
        let mut file_a = File::create(&path_a).unwrap();
        let mut file_b = File::create(&path_b).unwrap();
        file_a.write_all(b"this is file a").unwrap();
        file_b.write_all(b"this is file b").unwrap();
        for i in 0..1000_000 {
            let line = format!("this is line {}\n", i);
            file_a.write_all(line.as_bytes()).unwrap();
            file_b.write_all(line.as_bytes()).unwrap();
        }
        copy(&path_a, &path_a_copy).unwrap();
        (path_a, path_a_copy, path_b)
    }
    // cargo test tests::test_is_file_copy
    #[test]
    fn test_is_file_copy() {
        let (path_a, path_a_copy, path_b) = init_data();
        assert!(is_file_copy(&path_a, &path_a_copy));
        assert!(!is_file_copy(&path_a, &path_b));
    }
}
