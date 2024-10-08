use rand::Rng;
use std::fs::{self, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use std::{env, thread, process};

fn shred_file_parallel(path: PathBuf, passes: u32, threads: u32) -> io::Result<()> {
    let start_time = Instant::now();

    let file_path = Arc::new(path);
    let file_size = file_path.metadata()?.len();
    let chunk_size = file_size / (threads as u64);

    for pass in 0..passes {
        let mut handles = vec![];

        for i in 0..threads {
            let file_path = Arc::clone(&file_path);
            let handle = thread::spawn(move || -> io::Result<()> {
                let mut file = OpenOptions::new().write(true).open(&*file_path)?;
                let mut rng = rand::thread_rng();

                let start = i as u64 * chunk_size;
                let end = if i == threads - 1 {
                    file_size
                } else {
                    (i as u64 + 1) * chunk_size
                };

                file.seek(SeekFrom::Start(start))?;

                for _ in start..end {
                    let random_byte = rng.gen::<u8>();
                    file.write_all(&[random_byte])?;
                }

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

macro_rules! print_exit {
    ($message:expr) => {{
        // print error message
        eprintln!("error: {}", $message);
        // exit the process.
        process::exit(1);
    }};
}

fn get_src_path_from_args() -> PathBuf {
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

fn main() {
    let file_path = get_src_path_from_args();
    let passes = 5; // Number of overwrite passes

    match shred_file_parallel(file_path, passes, 4) {
        Ok(_) => {},
        Err(e) => print_exit!(e),
    }
}
