use file_shredder::get_src_path_from_args;
use file_shredder::print_exit;
use file_shredder::shred_file_parallel;

fn main() {
    let file_path = get_src_path_from_args();
    let passes = 5; // Number of overwrite passes

    match shred_file_parallel(file_path, passes, 4) {
        Ok(_) => {}
        Err(e) => print_exit!(e),
    }
}
