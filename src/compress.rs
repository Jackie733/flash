use anyhow::{Context, Result};
use clap::ValueEnum;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CompressionFormat {
    #[clap(name = "zip")]
    Zip,
    #[clap(name = "tar")]
    Tar,
    #[clap(name = "tar-gz")]
    TarGz,
}

impl CompressionFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            CompressionFormat::Zip => "zip",
            CompressionFormat::Tar => "tar",
            CompressionFormat::TarGz => "tar.gz",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CompressionFormat::Zip => "ZIP archive (compressed)",
            CompressionFormat::Tar => "TAR archive (uncompressed)",
            CompressionFormat::TarGz => "TAR.GZ archive (gzip compressed)",
        }
    }
}

impl Default for CompressionFormat {
    fn default() -> Self {
        CompressionFormat::Zip
    }
}

impl FromStr for CompressionFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "zip" => Ok(CompressionFormat::Zip),
            "tar" => Ok(CompressionFormat::Tar),
            "tar-gz " | "targe" | "tar.gz" | "tgz" => Ok(CompressionFormat::TarGz),
            _ => Err(format!("Unknown compression format: {}", s)),
        }
    }
}

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
        SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
    )?;
    zip.write_all(&buffer)?;
    zip.finish()?;

    Ok(())
}

pub fn compress(input_path: &str, ouput_path: &str, format: CompressionFormat) -> Result<()> {
    let input = Path::new(input_path);

    if !input.exists() {
        return Err(anyhow::anyhow!("Input path does not exist: {}", input_path));
    }

    match format {
        CompressionFormat::Zip => {
            if input.is_file() {
                compress_file_to_zip(input_path, ouput_path)
                    .with_context(|| "Failed to compress file to ZIP")?;
            } else if input.is_dir() {
                compress_folder_to_zip(input_path, ouput_path)
                    .with_context(|| "Failed to compress folder to ZIP")?;
            }
        }
        CompressionFormat::Tar => {
            compress_to_tar(input_path, ouput_path)
                .with_context(|| "Failed to compress file to TAR")?;
        }
        CompressionFormat::TarGz => {
            compress_to_tar_gz(input_path, ouput_path)
                .with_context(|| "Failed to compress file to TAR.GZ")?;
        }
    }

    Ok(())
}

fn compress_to_tar(input_path: &str, output_path: &str) -> Result<()> {
    let file = File::create(output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path))?;
    let mut builder = tar::Builder::new(file);
    let input = Path::new(input_path);
    if input.is_file() {
        builder
            .append_path_with_name(input, input.file_name().unwrap())
            .with_context(|| format!("Failed to add file to TAR: {}", input_path))?;
    } else if input.is_dir() {
        builder
            .append_dir_all(".", input_path)
            .with_context(|| format!("Failed to add directory to TAR: {}", input_path))?;
    }

    builder
        .finish()
        .with_context(|| "Failed to finish TAR archive")?;

    Ok(())
}

fn compress_to_tar_gz(input_path: &str, output_path: &str) -> Result<()> {
    let file = File::create(output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path))?;
    let gz_encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut builder = tar::Builder::new(gz_encoder);

    let input = Path::new(input_path);
    if input.is_file() {
        builder
            .append_path_with_name(input_path, input.file_name().unwrap())
            .with_context(|| format!("Failed to add file to TAR.GZ: {}", input_path))?;
    } else if input.is_dir() {
        builder
            .append_dir_all(".", input_path)
            .with_context(|| format!("Failed to add directory to TAR.GZ: {}", input_path))?;
    }

    builder
        .finish()
        .with_context(|| "Failed to finalize TAR.GZ archive")?;

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
                SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
            zip.write_all(&buffer)?;
        } else if path.is_dir() {
            let dir_name = format!("{}/", name.to_string_lossy());
            zip.add_directory(
                dir_name,
                SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
        }
    }
    zip.finish()?;

    Ok(())
}
