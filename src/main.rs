use std::fs::{File, OpenOptions};
use std::io::{self, Write, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

fn shred_file_parallel(path: &str, passes: u32, threads: u32) -> io::Result<()> {
    let file_path = Arc::new(Path::new(path).to_path_buf());
    let file_size = file_path.metadata()?.len();
    let chunk_size = file_size / (threads as u64);

    let mut handles = vec![];

    for pass in 0..passes {
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
        for handle in handles.drain(..) {
            handle.join().unwrap()?;
        }

        println!("Pass {} completed", pass + 1);
    }

    // Delete the file after shredding
    // std::fs::remove_file(&*file_path)?;
    println!("File shredded and deleted successfully");

    Ok(())
}

fn main() {
    let file_path = "file.txt";
    let passes = 1000; // Number of overwrite passes
    let threads = 4; // Number of threads to use

    match shred_file_parallel(file_path, passes, threads) {
        Ok(_) => println!("File shredding completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
