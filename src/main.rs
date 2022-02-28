
use std::path::PathBuf;
use clap::Parser;

/// 
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


#[derive(Debug, serde::Deserialize)]
struct LsblkJson {
    blockdevices: Vec<LsblkInner>
}

#[derive(Debug, serde::Deserialize)]
struct LsblkInner {
    pkname: Option<String>,
    kname: String,
    path: String,
}

fn zfs_find_partitions_in_pool(pool_name: &str) {
    let mut lzfs = libzfs::libzfs::Libzfs::new();
    
    let pool = lzfs.pool_by_name(pool_name).expect("Pool retreval failed");

    match pool.vdev_tree() {
        Ok(vdev) => { 
            let v = vdev_list_partitions(&vdev); 
            for i in v.iter() {
                let output = lsblk_lookup_dev(i);
                let first_dev = output.blockdevices.first().expect("expected first element");
                let p_no = get_dev_partition_number(&first_dev.kname);
                match &first_dev.pkname {
                    Some(pkname) => println!("{pkname} {p_no}"),
                    _ => {},
                }
            }
        },
        Err(e) => { eprintln!("Failed: {e}"); }
    };
}


fn get_dev_partition_number(dev_name: &str) -> String {
    let sysfs_path: std::path::PathBuf = ["/sys/class/block", dev_name, "partition"].iter().collect();
    let mut fin = std::fs::File::open(sysfs_path).expect("");
    
    use std::io::Read;
    
    let mut buf_str = String::new();
    fin.read_to_string(&mut buf_str);

    let buf_str = buf_str.trim().to_owned();
    buf_str

    
}

fn lsblk_lookup_dev(path: &std::path::Path) -> LsblkJson {
    let output = std::process::Command::new("lsblk")
        .args(&[ "-o", "PKNAME,KNAME,PATH", "--json"])
        .arg(path.as_os_str())
        .output().unwrap();

    serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap()
}

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
    fn test_lsblk_json() {
        let data = r#"
            {
                "blockdevices": [
                {
                    "pkname": null,
                    "kname": "vda",
                    "path": "/dev/vda"
                },{
                    "pkname": "vda",
                    "kname": "vda1",
                    "path": "/dev/vda1"
                },{
                    "pkname": "vda",
                    "kname": "vda2",
                    "path": "/dev/vda2"
                },{
                    "pkname": "vda",
                    "kname": "vda3",
                    "path": "/dev/vda3"
                }
                ]
            }
        "#;

        let json: LsblkJson = serde_json::from_str(&data).expect("");
    }

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
        
        vdev_list_partitions(&vdev);
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

        vdev_list_partitions(&vdev);
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

        vdev_list_partitions(&vdev);
    }
}
