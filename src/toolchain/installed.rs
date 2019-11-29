use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use semver::{Version, VersionReq};

use crate::{constants::TOOLCHAIN_FILE, toolchain::get_python_versions_from_path, Result};

#[derive(Debug, Clone, failure::Fail)]
#[fail(display = "Python version {} not found!", version)]
pub struct ToolchainNotInstalled {
    version: VersionReq,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InstalledToolchain {
    pub location: PathBuf,
    pub version: Version,
}

#[derive(Debug)]
pub struct NotInstalledToolchain {
    pub version: Option<VersionReq>,
    pub location: Option<PathBuf>,
}

impl InstalledToolchain {
    pub fn from_path<P>(path: P) -> Option<InstalledToolchain>
    where
        P: AsRef<Path>,
    {
        let versions_found = get_python_versions_from_path(path.as_ref());
        log::debug!("versions_found: {:?}", versions_found);

        let highest_version = versions_found.into_iter().max_by(|x, y| (x.0.cmp(&y.0)))?;
        log::debug!("highest_version: {:?}", highest_version);

        Some(InstalledToolchain {
            version: highest_version.0,
            location: highest_version.1,
        })
    }

    pub fn is_custom_install(&self) -> bool {
        match self.location.parent() {
            None => {
                log::error!("Cannot get parent directory of {:?}", self.location);
                false
            }
            Some(parent) => parent.join(crate::INFO_FILE).exists(),
        }
    }

    pub fn save_version(&self) -> Result<usize> {
        let version = format!("{}", VersionReq::exact(&self.version));
        save(&version, TOOLCHAIN_FILE)
    }

    pub fn save_path(&self) -> Result<usize> {
        let location = format!("{}", self.location.display());
        save(&location, TOOLCHAIN_FILE)
    }
}

fn save<P>(content: &str, path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    log::debug!("Writing toolchain selection to file {:?}", path.as_ref());

    let mut output = File::create(&path)?;
    let l1 = output.write(content.as_bytes())?;
    let l2 = output.write(b"\n")?;
    Ok(l1 + l2)
}