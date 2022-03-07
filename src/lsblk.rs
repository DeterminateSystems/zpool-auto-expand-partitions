use crate::errors::Result;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct LsblkJson {
    pub blockdevices: Vec<LsblkInner>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct LsblkInner {
    pub pkname: Option<String>,
    pub kname: String,
    pub path: String,
}

pub fn lsblk_lookup_dev(path: &std::path::Path) -> Result<LsblkJson> {
    let output = std::process::Command::new("lsblk")
        .args(&["-o", "PKNAME,KNAME,PATH", "--json"])
        .arg(path.as_os_str())
        .output()?;

    Ok(serde_json::from_str(&String::from_utf8(output.stdout)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsblk_json_output_deserialize() {
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

        let json: LsblkJson = serde_json::from_str(&data).unwrap();

        let cmp = LsblkJson {
            blockdevices: vec![
                LsblkInner {
                    pkname: None,
                    kname: "vda".into(),
                    path: "/dev/vda".into(),
                },
                LsblkInner {
                    pkname: Some("vda".into()),
                    kname: "vda1".into(),
                    path: "/dev/vda1".into(),
                },
                LsblkInner {
                    pkname: Some("vda".into()),
                    kname: "vda2".into(),
                    path: "/dev/vda2".into(),
                },
                LsblkInner {
                    pkname: Some("vda".into()),
                    kname: "vda3".into(),
                    path: "/dev/vda3".into(),
                },
            ],
        };

        assert_eq!(cmp, json);
    }
}
