

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Options {
    zpool_name: String,
}

fn main() {
    let options = Options::parse();
    let mut lzfs = libzfs::libzfs::Libzfs::new();

    let pool = lzfs.pool_by_name(&options.zpool_name).expect("Pool retreval failed");
    let cache = blkid::cache::Cache::new().expect("Failed to open blkid cache");
    match pool.vdev_tree() {
        Ok(vdev) => vdev_process(vdev),
        Err(e) => println!("Failed: {e}")
    };
}

fn vdev_process(vdev: libzfs::vdev::VDev, cache: &blkid::cache::Cache) {
    use libzfs::vdev::VDev;
    match vdev {
        VDev::Root { children, spares, cache } => {
            let a = children.iter().chain(spares.iter()).chain(cache.iter()).for_each(|v| vdev_disks(v, cache));
        },
        _ => {},
    };
}

fn vdev_disks(vdev: &libzfs::vdev::VDev, cache: &blkid::cache::Cache) {
    use libzfs::vdev::VDev;
    if matches!(vdev, VDev::Disk { whole_disk: Some(false), .. }) {
        dbg!(vdev);

    }
}
