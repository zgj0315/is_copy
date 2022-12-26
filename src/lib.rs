use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub fn is_file_copy(input_path: &Path, output_path: &Path) -> bool {
    if input_path == output_path {
        log::info!("same file, {:?}", input_path);
        return true;
    }
    let input_size = fs::metadata(input_path).unwrap().len();
    let output_size = fs::metadata(output_path).unwrap().len();
    if input_size != output_size {
        log::info!("diff size, {:?} and {:?}", input_path, output_path);
        return false;
    }
    let mut file = File::open(input_path).unwrap();
    let mut buf = [0u8; 1024];
    file.read(&mut buf).unwrap();
    let input_md5 = format!("{:X}", md5::compute(buf));
    let mut file = File::open(output_path).unwrap();
    let mut buf = [0u8; 1024];
    file.read(&mut buf).unwrap();
    let output_md5 = format!("{:X}", md5::compute(buf));
    if input_md5 == output_md5 {
        log::info!("same md5, {:?} and {:?}", input_path, output_path);
        return true;
    } else {
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

    use crate::is_file_copy;

    fn init_data() -> (PathBuf, PathBuf, PathBuf) {
        let path_data = Path::new("./data");
        if path_data.exists() {
            fs::remove_dir_all(path_data).unwrap();
        }
        fs::create_dir_all(path_data).unwrap();
        let path_a = path_data.join("file_a.txt");
        let path_a_copy = path_data.join("file_a_copy.txt");
        let path_b = path_data.join("file_b.txt");

        let mut file_a = File::create(&path_a).unwrap();
        file_a.write_all(b"this is file a").unwrap();
        copy(&path_a, &path_a_copy).unwrap();
        let mut file_b = File::create(&path_b).unwrap();
        file_b.write_all(b"this is file b").unwrap();
        (path_a, path_a_copy, path_b)
    }
    // cargo test tests::test_is_file_copy -- --nocapture
    #[test]
    fn test_is_file_copy() {
        let (path_a, path_a_copy, path_b) = init_data();
        assert!(is_file_copy(&path_a, &path_a_copy));
        println!(
            "{:?} and {:?} is file copy: {}",
            &path_a,
            &path_a_copy,
            is_file_copy(&path_a, &path_a_copy)
        );
        assert!(!is_file_copy(&path_a, &path_b));
        println!(
            "{:?} and {:?} is file copy: {}",
            &path_a,
            &path_b,
            is_file_copy(&path_a, &path_b)
        );
    }
}
