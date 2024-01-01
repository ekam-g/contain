use encryption::file::encrypt_file;

pub mod encryption;

fn main() {
    encryption::test();
    encrypt_file();
}
