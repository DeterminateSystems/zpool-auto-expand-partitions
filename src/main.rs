use clap::Parser;

mod errors;
mod grow;
mod identify;
mod lsblk;

use crate::errors::Result;
use crate::identify::zfs_find_partitions_in_pool;

#[derive(Debug, Parser)]
#[clap(about, long_about = None)]
struct Options {
    /// Specified ZPool name to lookup in ZFS
    zpool_name: String,

    /// Automatically grow all candidate partitions (true|false)
    #[clap(long, parse(try_from_str = true_or_false), default_value_t)]
    automatically_grow: bool,

    /// Don't make any changes (true|false)
    #[clap(long, parse(try_from_str = true_or_false), default_value_t)]
    dry_run: bool,
}

fn true_or_false(s: &str) -> Result<bool, &'static str> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("expected `true` or `false`"),
    }
}

fn main() -> Result<()> {
    let options = Options::parse();

    let disk_parts = zfs_find_partitions_in_pool(&options.zpool_name)?;

    for disk in &disk_parts {
        println!("{} {}", disk.parent_path.display(), disk.partition);

        if options.automatically_grow {
            println!("Growing the partition and expanding the vdev...");
            grow::grow(&options.zpool_name, disk, options.dry_run).unwrap();
            println!("...ok.");

            println!("");
        }
    }

    Ok(())
}
