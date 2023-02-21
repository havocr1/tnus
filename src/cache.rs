use anyhow::Result;
use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};
use tracing::debug;

use crate::defines::{app_cache_dir, HACPACK, HACTOOL};

#[derive(Debug, Clone, Copy)]
pub enum Cache {
    Hacpack,
    Hactool,
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(target_os = "windows")]
            Cache::Hacpack => write!(f, "hacpack.exe"),
            #[cfg(target_os = "windows")]
            Cache::Hactool => write!(f, "hactool.exe"),
            #[cfg(any(target_os = "linux", target_os = "android"))]
            Cache::Hacpack => write!(f, "hacpack"),
            #[cfg(any(target_os = "linux", target_os = "android"))]
            Cache::Hactool => write!(f, "hactool"),
        }
    }
}

impl Cache {
    /// Saves the given file as a cache for `self`.
    ///
    /// Overwrited the previous cache in the process if any.
    pub fn from<P: AsRef<Path>>(self, path: P) -> Result<Self> {
        debug!("Copying {:?} as cache for {:?}", path.as_ref(), self);

        let cache_dir = app_cache_dir();
        fs::create_dir_all(&cache_dir)?;
        fs::copy(path.as_ref(), cache_dir.join(self.to_string()))?;

        Ok(self)
    }
    /// Returns the path to the embedded resource.
    ///
    /// Cache is used if it exists else the embedded data is written to a file
    /// and the path is returned.
    pub fn path(&self) -> Result<PathBuf> {
        let cache_dir = app_cache_dir();
        fs::create_dir_all(&cache_dir)?;

        let file_name = self.to_string();
        for entry in fs::read_dir(&cache_dir)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy() == file_name {
                // return cache if exists
                return Ok(entry.path());
            }
        }

        // Extract the embedded files to cache folder
        let path = cache_dir.join(file_name);
        let mut file = fs::File::create(&path)?;
        file.write_all(self.as_bytes())?;

        Ok(path)
    }
    fn as_bytes(&self) -> &'static [u8] {
        match self {
            Cache::Hacpack => HACPACK,
            Cache::Hactool => HACTOOL,
        }
    }
}
