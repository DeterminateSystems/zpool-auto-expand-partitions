

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
        Ok(vdev) => vdev_process(vdev, &cache),
        Err(e) => println!("Failed: {e}")
    };
}

fn vdev_process(vdev: libzfs::vdev::VDev, cache: &blkid::cache::Cache) {
    use libzfs::vdev::VDev;
    match vdev {
        VDev::Root { children, .. } => {
            let a = children.iter()
                // .chain(spares.iter())
                // .chain(cache.iter())
                .for_each(|v| vdev_disks(v, cache));
        },
        _ => {},
    };
}

fn vdev_disks(vdev: &libzfs::vdev::VDev, cache: &blkid::cache::Cache) {
    use libzfs::vdev::VDev;
    match vdev {
        VDev::Disk { path, whole_disk: Some(false), state, .. } if state == "ONLINE" => {
            // dbg!(path);
            dbg!(&vdev);
            dbg!(path.file_name());
            dbg!({
                let mut p: std::path::PathBuf = "/sys/class/block".into();
                p.push(path.file_name().unwrap());
                p.push("partition");
                std::fs::File::open(p)
            });
            dbg!(cache.get_dev(path.to_str().unwrap(), blkid::dev::GetDevFlags::empty()).unwrap().name());

            use blkid::prober::{Prober, ProbeState};
            let prober = Prober::new_from_filename(path).unwrap();
            prober.enable_partitions(true);

            if matches!(prober.do_full_probe(), Ok(ProbeState::Success)) {
                dbg!(prober.lookup_value("TYPE"));
                dbg!(prober.lookup_value("PTTYPE"));
                let ptlist = prober
                    .part_list();
                dbg!(ptlist.as_ref().err());
                dbg!(
                        ptlist.ok().and_then(|v| v.get_table())
                        // .and_then(|v| v.get_parent())
                );
            }

        },
        _ => {},
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        vdev_disks()
    }
}