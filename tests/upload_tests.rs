#[cfg(test)]
mod tests {
    use flash::upload;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_upload_via_sftp_local_mock() {
        let test_file = "mock_upload.txt";
        let mut f = File::create(test_file).unwrap();
        writeln!(f, "mock").unwrap();
        let result = upload::upload_via_sftp(
            "127.0.0.1",
            22,
            "user",
            "pass",
            test_file,
            "/tmp/mock_upload.txt.zip",
        );
        assert!(result.is_err());
        fs::remove_file(test_file).unwrap();
    }
}
