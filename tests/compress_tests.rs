#[cfg(test)]
mod tests {
    use flash::compress;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_compress_file_to_zip() {
        let test_file = "test_file.txt";
        let zip_file = "test_file.txt.zip";
        let mut f = File::create(test_file).unwrap();
        writeln!(f, "hello world").unwrap();
        compress::compress_file_to_zip(test_file, zip_file).unwrap();
        assert!(fs::metadata(zip_file).is_ok());
        fs::remove_file(test_file).unwrap();
        fs::remove_file(zip_file).unwrap();
    }

    #[test]
    fn test_compress_folder_to_zip() {
        let test_dir = "test_dir";
        let zip_file = "test_dir.zip";
        fs::create_dir(test_dir).unwrap();
        let mut f = File::create(format!("{}/a.txt", test_dir)).unwrap();
        writeln!(f, "abc").unwrap();
        compress::compress_folder_to_zip(test_dir, zip_file).unwrap();
        assert!(fs::metadata(zip_file).is_ok());
        fs::remove_file(format!("{}/a.txt", test_dir)).unwrap();
        fs::remove_dir(test_dir).unwrap();
        fs::remove_file(zip_file).unwrap();
    }
}
