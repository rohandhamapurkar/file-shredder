use rand::Rng;
use std::fs::{self, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use std::{env, thread};

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
    return vec
}

pub fn shred_file_parallel(path: PathBuf, passes: u32, threads: u32) -> io::Result<()> {
    let start_time = Instant::now();

    let file_path = Arc::new(path);
    let file_size = file_path.metadata()?.len();
    let chunk_size = file_size / (threads as u64);

    for pass in 0..passes {
        println!("Executing pass {}", pass + 1);
        let mut handles = vec![];

        for i in 0..threads {
            let file_path = Arc::clone(&file_path);
            let handle = thread::spawn(move || -> io::Result<()> {
                let mut file = OpenOptions::new().write(true).open(&*file_path)?;

                let start: u64 = i as u64 * chunk_size;
                let end = if i == threads - 1 {
                    file_size
                } else {
                    (i as u64 + 1) * chunk_size
                };
                file.seek(SeekFrom::Start(start))?;

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
    std::fs::remove_file(&*file_path)?;

    let total_duration = start_time.elapsed();
    println!("File shredded and deleted in {:?}", total_duration);

    Ok(())
}

pub fn get_src_path_from_args() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_exit!("Please provide folder or file path to me!");
    }

    let src_path = PathBuf::from(&args[1]);

    if !fs::exists(&src_path).unwrap_or_else(|e| {
        print_exit!(e);
    }) {
        print_exit!("Path doesn't exist!");
    }

    return src_path;
}
