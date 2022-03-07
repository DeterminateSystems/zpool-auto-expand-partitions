use clap::Parser;

mod errors;
mod identify;
mod lsblk;

use crate::errors::Result;
use crate::identify::zfs_find_partitions_in_pool;

#[derive(Debug, Parser)]
#[clap(about, long_about = None)]
struct Options {
    /// Specified ZPool name to lookup in ZFS
    zpool_name: String,

    /// Automatically grow all candidate partitions
    #[clap(long)]
    automatically_grow: bool,

    /// Don't make any changes
    #[clap(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();

    let disk_parts = zfs_find_partitions_in_pool(&options.zpool_name)?;

    for disk in &disk_parts {
        println!("{} {}", disk.parent_path.display(), disk.partition);
    }

    Ok(())
}
