use std::{fs, path::PathBuf};

pub trait PathImpls {
    fn os_bin_name(&self) -> Self;
    fn ensure_parent(self) -> Self;
    fn ensure_executable(self) -> std::io::Result<PathBuf>;
    fn ensure_nonexist(self) -> Self;
}

impl PathImpls for PathBuf {
    fn os_bin_name(&self) -> Self {
        #[allow(unused_mut)]
        let mut ret = self.clone();

        #[cfg(target_os = "windows")]
        {
            if !ret.ends_with(".exe") {
                ret = ret.with_added_extension("exe");
            }
        }
        ret
    }

    fn ensure_parent(self) -> Self {
        fs::create_dir_all(self.parent().unwrap()).unwrap();
        self
    }

    fn ensure_executable(self) -> std::io::Result<PathBuf> {
        if !self.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                self.to_string_lossy(),
            ));
        }
        #[cfg(unix)]
        {
            use std::fs;
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self)?.permissions();
            perms.set_mode(perms.mode() | 0o111);
            fs::set_permissions(&self, perms)?;
        }
        Ok(self)
    }

    fn ensure_nonexist(self) -> Self {
        if self.is_dir() {
            fs::remove_dir_all(&self).unwrap();
        } else if self.exists() {
            fs::remove_file(&self).unwrap();
        }
        self
    }
}
