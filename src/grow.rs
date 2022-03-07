use crate::identify::DriveData;

use std::fmt;
use std::process::{Command, ExitStatus, Output};

pub fn grow(pool_name: &str, disk: &DriveData, dry_run: bool) -> Result<(), GrowError> {
    let mut growcmd = Command::new("growpart");
    growcmd.arg(&disk.parent_path);
    growcmd.arg(&disk.partition);
    if dry_run {
        growcmd.arg("--dry-run");
    }

    let output = growcmd
        .output()
        .map_err(|e| GrowError::GrowpartSpawnError(e))?;
    if !output.status.success() {
        if output.status.code() == Some(1) && output.stdout.starts_with(b"NOCHANGE") {
            println!("Growpart indicates the partition cannot be grown. Running online -e anyway.");
        } else {
            return Err(GrowError::GrowpartFailed(output));
        }
    }

    let mut expandcmd = Command::new("zpool");
    expandcmd.args(&["online", "-e"]);
    expandcmd.arg(pool_name);
    expandcmd.arg(&disk.path);

    if dry_run {
        println!("Would run: {:?}", expandcmd);
    } else {
        let status = expandcmd
            .status()
            .map_err(|e| GrowError::ZpoolSpawnError(e))?;
        if !status.success() {
            return Err(GrowError::ZpoolOnlineFailed(status));
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum GrowError {
    GrowpartSpawnError(std::io::Error),
    GrowpartFailed(Output),

    ZpoolSpawnError(std::io::Error),
    ZpoolOnlineFailed(ExitStatus),
}

impl fmt::Display for GrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GrowError")
    }
}

impl std::error::Error for GrowError {}
