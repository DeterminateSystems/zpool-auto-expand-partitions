

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
    match pool.vdev_tree() {
        Ok(vdev) => vdev_process(vdev),
        Err(e) => println!("Failed: {e}")
    };
}

fn vdev_process(vdev: libzfs::vdev::VDev) {
    
}
