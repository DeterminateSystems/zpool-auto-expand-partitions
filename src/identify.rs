use std::path::PathBuf;

use crate::errors::Result;
use crate::lsblk;

#[derive(Debug)]
pub struct DriveData {
    pub path: PathBuf,
    pub parent: String,
    pub parent_path: PathBuf,
    pub name: String,
    pub partition: String,
}

pub fn zfs_find_partitions_in_pool(pool_name: &str) -> Result<Vec<DriveData>> {
    let mut lzfs = libzfs::libzfs::Libzfs::new();

    let pool = lzfs
        .pool_by_name(pool_name)
        .ok_or("Pool retrieval failed")?;

    let mut acc = vec![];
    match pool.vdev_tree() {
        Ok(vdev) => {
            let disks = vdev_list_partitions(&vdev);
            for disk_path in disks.iter() {
                let output = lsblk::lsblk_lookup_dev(disk_path)?;
                let first_dev = output
                    .blockdevices
                    .first()
                    .ok_or("expected first element of blockdevices")?;

                let p_no = get_dev_partition_number(&first_dev.kname)?;

                match &first_dev.pkname {
                    Some(pkname) => acc.push(DriveData {
                        partition: p_no,
                        parent: pkname.to_owned(),
                        parent_path: { ["/dev", pkname].iter().collect() },
                        path: disk_path.to_path_buf(),
                        name: first_dev.kname.to_owned(),
                    }),
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("Failed: {e}");
        }
    };

    Ok(acc)
}

fn get_dev_partition_number(dev_name: &str) -> Result<String> {
    let sysfs_path: PathBuf = ["/sys/class/block", dev_name, "partition"].iter().collect();
    let mut fin = std::fs::File::open(sysfs_path)?;

    use std::io::Read;

    let mut buf_str = String::new();
    fin.read_to_string(&mut buf_str)?;

    let buf_str = buf_str.trim().to_owned();
    Ok(buf_str)
}

fn vdev_list_partitions<'a>(vdev: &'a libzfs::vdev::VDev) -> Vec<&'a PathBuf> {
    let mut vec = vec![];
    vdev_find_partitions(vdev, &mut vec);
    vec
}

fn vdev_find_partitions<'a>(vdev: &'a libzfs::vdev::VDev, devs: &mut Vec<&'a PathBuf>) {
    use libzfs::vdev::VDev;
    match vdev {
        VDev::Disk {
            is_log: None | Some(false),
            whole_disk: Some(false),
            state,
            path,
            ..
        } if state == "ONLINE" => {
            devs.push(path);
        }

        VDev::Root { children, .. }
        | VDev::Mirror { children, .. }
        | VDev::RaidZ { children, .. } => {
            children.iter().for_each(|i| vdev_find_partitions(i, devs))
        }

        VDev::Disk { .. } => {}

        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vdev_tank_example() {
        use libzfs::vdev::VDev;

        let vdev = VDev::Root {
            children: vec![VDev::Disk {
                whole_disk: Some(false),
                state: "ONLINE".into(),
                path: "/dev/vda3".into(),
                guid: None,
                dev_id: None,
                phys_path: None,
                is_log: None,
            }],
            spares: vec![],
            cache: vec![],
        };

        let disks = vdev_list_partitions(&vdev);
        assert_eq!(disks, &[&std::path::PathBuf::from("/dev/vda3")])
    }

    #[test]
    fn test_multiple_disks() {
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
                    state: "OFFLINE".into(),
                    path: "vdc1".into(),
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
                },
            ],
            spares: vec![],
            cache: vec![],
        };

        use std::path::PathBuf;
        assert_eq!(
            vdev_list_partitions(&vdev),
            &[&PathBuf::from("vda1"), &PathBuf::from("vdb1")]
        );
    }

    #[test]
    fn test_multiple_disks_in_mirror() {
        use libzfs::vdev::VDev;

        let vdev = VDev::Root {
            children: vec![VDev::Mirror {
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
                    },
                ],
            }],
            spares: vec![],
            cache: vec![],
        };

        use std::path::PathBuf;
        assert_eq!(
            vdev_list_partitions(&vdev),
            &[&PathBuf::from("vda1"), &PathBuf::from("vdb1")]
        );
    }
}
