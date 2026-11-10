pub mod myfs;

fn main() {
    myfs::test_create_dir();
    myfs::create_files();
    myfs::remove_dir();
    myfs::read_file();
}
