use std::error::Error;

use file_shredder::get_args;
use file_shredder::print_exit;
use file_shredder::shred_file;
use file_shredder::shred_folder;
use file_shredder::errors::CustomError::PathNonExistErr;

fn main() -> Result<(), Box<dyn Error>> {
    let (src_path, passes) = get_args().unwrap_or_else(|e| {
        print_exit!(e);
    });


    let result: Result<(), Box<dyn Error>> = match () {
        _ if src_path.is_file() => shred_file(src_path, passes, 5),
        _ if src_path.is_dir() => shred_folder(src_path, passes, 5),
        _ => Err(Box::new(PathNonExistErr))
    };

    if let Err(e) = result {
        print_exit!(e)
    }

    Ok(())
}
