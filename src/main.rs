use file_shredder::get_src_path_from_args;
use file_shredder::print_exit;
use file_shredder::shred_file;
use file_shredder::shred_folder;

fn main() {
    let src_path = get_src_path_from_args();
    let passes = 5; // Number of overwrite passes

    if src_path.is_file() {
        if let Err(e) = shred_file(src_path, passes, 5) {
            print_exit!(e);
        }
    } else if src_path.is_dir() {
        if let Err(e) = shred_folder(src_path, passes, 5) {
            print_exit!(e);
        }
    } else {
        print_exit!(format!(
            "Source path is neither a file nor a directory: {}",
            src_path.display()
        ));
    }
}
