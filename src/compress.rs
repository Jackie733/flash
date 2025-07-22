use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::CompressionMethod;
use zip::write::FileOptions;

pub fn compress_file_to_zip(input_path: &str, output_path: &str) -> io::Result<()> {
    let path = Path::new(input_path);
    let file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let mut buffer = Vec::new();
    let mut f = File::open(path)?;
    f.read_to_end(&mut buffer)?;

    let file_name = path.file_name().unwrap().to_string_lossy();
    zip.start_file(
        file_name,
        FileOptions::default().compression_method(CompressionMethod::Deflated),
    )?;
    zip.write_all(&buffer)?;
    zip.finish()?;

    Ok(())
}

pub fn compress_folder_to_zip(folder_path: &str, output_zip: &str) -> io::Result<()> {
    let file = File::create(output_zip)?;
    let mut zip = zip::ZipWriter::new(file);

    let walkdir = WalkDir::new(folder_path);
    let folder_path = Path::new(folder_path);

    for entry in walkdir.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(folder_path).unwrap();

        if path.is_file() {
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;

            zip.start_file(
                name.to_string_lossy(),
                FileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
            zip.write_all(&buffer)?;
        } else if path.is_dir() {
            let dir_name = format!("{}/", name.to_string_lossy());
            zip.add_directory(
                dir_name,
                FileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
        }
    }
    zip.finish()?;

    Ok(())
}
