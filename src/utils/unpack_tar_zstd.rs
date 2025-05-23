use crate::failed;
use std::{
    fs,
    io::{self, BufReader},
    path::Path,
};
use tar::Archive;

pub fn unpack_tar_zstd<P: AsRef<Path>>(path: P, dist: P) -> Result<(), io::ErrorKind> {
    let path = path.as_ref();

    let Ok(file) = fs::File::open(path) else {
        return Err(io::ErrorKind::NotFound);
    };

    let buf_reader = BufReader::new(file);

    let Ok(zstd_decoder) = zstd::stream::read::Decoder::new(buf_reader) else {
        return Err(io::ErrorKind::InvalidData);
    };

    let mut archive = Archive::new(zstd_decoder);

    archive.unpack(dist).unwrap_or_else(|e| {
        failed!(
            "Failed to unpack",
            "Failed to unpack tar.zst package\n Caused by {}",
            e
        );
    });

    Ok(())
}
