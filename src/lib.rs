use rand::Rng;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use std::usize;
use std::{env, fs, io, thread, u32};
use walkdir::WalkDir;
mod errors;
use errors::CustomError::NotEnoughArgumentsErr;
use errors::CustomError::PathNonExistErr;

#[macro_export]
macro_rules! print_exit {
    ($message:expr) => {{
        // print error message
        eprintln!("error: {}", $message);
        // exit the process.
        std::process::exit(1);
    }};
}

fn generate_random_array(length: u64) -> Vec<u8> {
    let length_usize = usize::try_from(length).unwrap_or(usize::MAX);
    let mut vec = vec![0; length_usize];
    rand::thread_rng().fill(&mut vec[..]);
    return vec;
}

pub fn shred_file(path: PathBuf, passes: u32, threads: u32) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let file_path = Arc::new(path);
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_size = file_path.metadata()?.len();
    let chunk_size = file_size / (threads as u64);

    println!("Started shredding {}", file_name);

    for pass in 0..passes {
        // println!("Executing pass {}", pass + 1);
        let mut handles = vec![];

        for i in 0..threads {
            let file_path = Arc::clone(&file_path);
            let handle = thread::spawn(move || -> io::Result<()> {
                let mut file = fs::OpenOptions::new().write(true).open(&*file_path)?;

                let start: u64 = i as u64 * chunk_size;
                let end = if i == threads - 1 {
                    file_size
                } else {
                    (i as u64 + 1) * chunk_size
                };
                file.seek(io::SeekFrom::Start(start))?;

                let data_length = end - start;
                let random_data = generate_random_array(data_length);

                file.write_all(&random_data)?;

                file.flush()?;
                Ok(())
            });
            handles.push(handle);
        }

        // Wait for all threads in this pass to complete
        for handle in handles {
            handle.join().unwrap()?;
        }

        println!("Pass {} completed", pass + 1);
    }

    // Delete the file after shredding
    fs::remove_file(&*file_path)?;

    let total_duration = start_time.elapsed();
    println!("{} shredded and deleted in {:?}", file_name, total_duration);

    Ok(())
}

pub fn shred_folder(dir_path: PathBuf, passes: u32, threads: u32) -> Result<(), Box<dyn Error>> {
    let mut deque: VecDeque<PathBuf> = VecDeque::new();
    for entry in WalkDir::new(&dir_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        // skip the root directory
        if path.eq(&dir_path) {
            deque.push_front(path.to_path_buf());
            continue;
        }

        if path.is_file() {
            shred_file(path.to_path_buf(), passes, threads)?
        } else if path.is_dir() {
            deque.push_front(path.to_path_buf());
        }
    }

    // delete all the directories after shredding files.
    fs::remove_dir_all(dir_path)?;
    println!("Deleted directories");

    Ok(())
}

pub fn get_args() -> Result<(PathBuf, u32), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path> [passes]", args[0]);
        return Err(Box::new(NotEnoughArgumentsErr));
    }

    let mut src_path: PathBuf = PathBuf::new();
    let mut passes: u32 = 5;

    if args.len() == 2 {
        src_path = PathBuf::from(&args[1]);
        if !fs::exists(&src_path)? {
            return Err(Box::new(PathNonExistErr));
        }
    }
    if args.len() == 3 {
        passes = args[2].parse::<u32>()?;
    }

    return Ok((src_path, passes));
}
