

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, long_about = None)]
struct Options {
    /// Specified ZPool name to lookup in ZFS
    zpool_name: String,
}

fn main() {
    let options = Options::parse();
    zfs_find_partitions_in_pool(&options.zpool_name);
}

fn zfs_find_partitions_in_pool(pool_name: &str) {
    let mut lzfs = libzfs::libzfs::Libzfs::new();
    
    let pool = lzfs.pool_by_name(pool_name).expect("Pool retreval failed");

    match pool.vdev_tree() {
        Ok(vdev) => { vdev_find_partitions(&vdev)},
        Err(e) => eprintln!("Failed: {e}")
    };
}

fn vdev_find_partitions(vdev: &libzfs::vdev::VDev) {

fn vdev_list_partitions<'a>(vdev: &'a libzfs::vdev::VDev) -> Vec<&'a PathBuf> {
    let mut vec = vec![];
    vdev_find_partitions(vdev, &mut vec);
    vec
}

fn vdev_find_partitions<'a>(vdev: &'a libzfs::vdev::VDev, devs: &mut Vec<&'a PathBuf>) {
    use libzfs::vdev::VDev;
    match vdev {
        VDev::Disk { is_log: None | Some(false), whole_disk: Some(false), state, path, .. } if state == "ONLINE" => {
            devs.push(path);
        },
        
        VDev::Root { children, .. } 
        | VDev::Mirror { children, .. } 
        | VDev::RaidZ { children, .. } => {
            children.iter()
                // .chain(spares.iter())
                // .chain(cache.iter())
                .for_each(|i| vdev_find_partitions(i, devs));
        },

        _ => { eprintln!(" unimplemented "); },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vdev_tank() {
        use libzfs::vdev::VDev;

        let vdev = VDev::Root {
            children: vec![
                VDev::Disk {
                    whole_disk: Some(false),
                    state: "ONLINE".into(),
                    path: "/dev/vda3".into(),
                    guid: None,
                    dev_id: None,
                    phys_path: None,
                    is_log: None,
                }
            ],
            spares: vec![],
            cache: vec![],
        };

        vdev_find_partitions(&vdev);
    }

    #[test]
    fn test_vdevs_tank() {
        use libzfs::vdev::VDev;

        let vdev = VDev::Root {
            children: vec![
                VDev::Disk {
                    whole_disk: Some(false),
                    state: "ONLINE".into(),
                    path: "vda1".into(),
                    guid: None,
                    dev_id: None,
                    phys_path: None,
                    is_log: None,
                },
                VDev::Disk {
                    whole_disk: Some(false),
                    state: "ONLINE".into(),
                    path: "vdb1".into(),
                    guid: None,
                    dev_id: None,
                    phys_path: None,
                    is_log: None,
                }
            ],
            spares: vec![],
            cache: vec![],
        };

        vdev_find_partitions(&vdev);
    }

    #[test]
    fn test_vdevs_mirror() {
        use libzfs::vdev::VDev;

        let vdev = VDev::Root {
            children: vec![
                VDev::Mirror {
                    is_log: None,
                    children: vec![

                        VDev::Disk {
                            whole_disk: Some(false),
                            state: "ONLINE".into(),
                            path: "vda1".into(),
                            guid: None,
                            dev_id: None,
                            phys_path: None,
                            is_log: None,
                        },
                        VDev::Disk {
                            whole_disk: Some(false),
                            state: "ONLINE".into(),
                            path: "vdb1".into(),
                            guid: None,
                            dev_id: None,
                            phys_path: None,
                            is_log: None,
                        }
                    ],
                }
            ],
            spares: vec![],
            cache: vec![],
        };

        vdev_find_partitions(&vdev);
    }
}
