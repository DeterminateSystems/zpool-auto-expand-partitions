# `zpool-auto-expand-partitions`

`zpool_part_disks` is a tool that aims to expand all partitions in a specified
zpool to fill the available space.

## Usage

```
$ zpool_part_disks --help
$ zpool_part_disks --automatically-grow zpool
$ zpool_part_disks --automatically-grow --dry-run zpool
```

## Known Issues
* [rust-libzfs](https://github.com/whamcloud/rust-libzfs/) [does not check that the pointer returned from libzfs_init is non-null](https://github.com/whamcloud/rust-libzfs/blob/master/libzfs/src/libzfs.rs#L33-L37); resulting in a segfault on systems that do not have zfs available ([whamcloud/rust-libzfs#59](https://github.com/whamcloud/rust-libzfs/issues/59/))

## Minimum Supported Rust Version (MSRV)
We use cargo edition 2021, which requires at least 1.56.0.

# License
[Apache Software License 2.0](./LICENSE)
