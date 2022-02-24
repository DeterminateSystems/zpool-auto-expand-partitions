fn main() {
    let mut lzfs = libzfs::libzfs::Libzfs::new();

    let pool = lzfs.pool_by_name("tank").expect("expected pool");
    dbg!(pool.vdev_tree());
}
