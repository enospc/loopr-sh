use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::{LooprError, LooprResult};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn ensure_dir(path: &Path, mode: u32) -> LooprResult<()> {
    fs::create_dir_all(path)
        .map_err(|err| LooprError::new(format!("create dir {}: {}", path.display(), err)))?;
    #[cfg(unix)]
    {
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
            .map_err(|err| LooprError::new(format!("chmod {}: {}", path.display(), err)))?;
    }
    Ok(())
}

pub fn copy_file(src: &Path, dst: &Path, mode: u32) -> LooprResult<()> {
    if let Some(parent) = dst.parent() {
        ensure_dir(parent, 0o755)?;
    }
    let mut input = File::open(src)
        .map_err(|err| LooprError::new(format!("open {}: {}", src.display(), err)))?;
    let mut output = File::create(dst)
        .map_err(|err| LooprError::new(format!("create {}: {}", dst.display(), err)))?;

    std::io::copy(&mut input, &mut output).map_err(|err| {
        LooprError::new(format!(
            "copy {} to {}: {}",
            src.display(),
            dst.display(),
            err
        ))
    })?;
    output
        .sync_all()
        .map_err(|err| LooprError::new(format!("sync {}: {}", dst.display(), err)))?;
    #[cfg(unix)]
    {
        fs::set_permissions(dst, fs::Permissions::from_mode(mode))
            .map_err(|err| LooprError::new(format!("chmod {}: {}", dst.display(), err)))?;
    }
    Ok(())
}

pub fn copy_dir(src: &Path, dst: &Path) -> LooprResult<()> {
    let entries = fs::read_dir(src)
        .map_err(|err| LooprError::new(format!("read dir {}: {}", src.display(), err)))?;
    for entry in entries {
        let entry = entry.map_err(|err| LooprError::new(format!("read dir entry: {}", err)))?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|err| LooprError::new(format!("metadata {}: {}", path.display(), err)))?;
        let target = dst.join(entry.file_name());
        if metadata.is_dir() {
            let mode = dir_mode(&metadata);
            ensure_dir(&target, mode)?;
            copy_dir(&path, &target)?;
        } else {
            let mode = file_mode(&metadata);
            copy_file(&path, &target, mode)?;
        }
    }
    Ok(())
}

pub fn write_file_atomic(path: &Path, data: &[u8], mode: u32) -> LooprResult<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    ensure_dir(dir, 0o755)?;

    let mut attempt = 0;
    let temp_path;
    let mut temp_file: File;

    loop {
        attempt += 1;
        if attempt > 10 {
            return Err(LooprError::new(format!(
                "create temp for {}: too many attempts",
                path.display()
            )));
        }
        let suffix = random_hex(6)?;
        let candidate = dir.join(format!(".tmp-{}", suffix));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                temp_path = candidate;
                temp_file = file;
                break;
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => {
                return Err(LooprError::new(format!(
                    "create temp for {}: {}",
                    path.display(),
                    err
                )));
            }
        }
    }

    temp_file
        .write_all(data)
        .map_err(|err| LooprError::new(format!("write temp for {}: {}", path.display(), err)))?;
    temp_file
        .sync_all()
        .map_err(|err| LooprError::new(format!("sync temp for {}: {}", path.display(), err)))?;
    #[cfg(unix)]
    {
        fs::set_permissions(&temp_path, fs::Permissions::from_mode(mode)).map_err(|err| {
            LooprError::new(format!("chmod temp for {}: {}", path.display(), err))
        })?;
    }
    drop(temp_file);
    fs::rename(&temp_path, path)
        .map_err(|err| LooprError::new(format!("rename temp for {}: {}", path.display(), err)))?;
    Ok(())
}

fn random_hex(bytes_len: usize) -> LooprResult<String> {
    let mut bytes = vec![0u8; bytes_len];
    getrandom::fill(&mut bytes).map_err(|err| LooprError::new(format!("random bytes: {}", err)))?;
    let mut out = String::with_capacity(bytes_len * 2);
    for byte in bytes {
        out.push(hex_digit(byte >> 4));
        out.push(hex_digit(byte & 0x0f));
    }
    Ok(out)
}

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => '0',
    }
}

#[cfg(unix)]
fn dir_mode(metadata: &fs::Metadata) -> u32 {
    metadata.permissions().mode()
}

#[cfg(unix)]
fn file_mode(metadata: &fs::Metadata) -> u32 {
    metadata.permissions().mode()
}

#[cfg(not(unix))]
fn dir_mode(_: &fs::Metadata) -> u32 {
    0o755
}

#[cfg(not(unix))]
fn file_mode(_: &fs::Metadata) -> u32 {
    0o644
}
