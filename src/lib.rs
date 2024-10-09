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
pub mod errors;
use errors::CustomError::NotEnoughArgumentsErr;
use errors::CustomError::PathNonExistErr;
use errors::CustomError::InvalidPassesErr;

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
    if passes == 0 {
        return Err(Box::new(InvalidPassesErr));
    }

    if threads == 0 {
        return Err(Box::new(PathNonExistErr));
    }

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
#[cfg(test)]
mod tests {

    use super::*;
    use std::{io::Read, path::Path};
    use tempfile::{NamedTempFile, TempDir};

    // fn cleanup_temp_file(path: &PathBuf) {
    //     fs::remove_file(path).expect("Failed to remove temp file");
    // }

    #[test]
    fn test_generate_random_array() {
        let result = generate_random_array(5);
        assert_eq!(result.len(), 5);
    }

    fn create_test_file(content: &[u8]) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content)
            .expect("Failed to write to temp file");
        file
    }

    #[test]
    fn test_successful_shred() -> Result<(), Box<dyn Error>> {
        let content = b"This is a test file for shredding.";
        let temp_file = create_test_file(content);
        let path = temp_file.path().to_path_buf();

        shred_file(path.clone(), 3, 2)?;

        assert!(!path.exists(), "File should be deleted after shredding");
        Ok(())
    }

    #[test]
    fn test_shred_large_file() -> Result<(), Box<dyn Error>> {
        let content = vec![0u8; 1024 * 1024 * 1]; // 5 MB file
        let temp_file = create_test_file(&content);
        let path = temp_file.path().to_path_buf();

        shred_file(path.clone(), 2, 4)?;

        assert!(
            !path.exists(),
            "Large file should be deleted after shredding"
        );
        Ok(())
    }

    #[test]
    fn test_shred_empty_file() -> Result<(), Box<dyn Error>> {
        let temp_file = create_test_file(&[]);
        let path = temp_file.path().to_path_buf();

        shred_file(path.clone(), 1, 1)?;

        assert!(
            !path.exists(),
            "Empty file should be deleted after shredding"
        );
        Ok(())
    }

    #[test]
    fn test_shred_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/file.txt");
        let result = shred_file(path, 1, 1);
        assert!(result.is_err(), "Shredding nonexistent file should fail");
    }

    #[test]
    fn test_shred_with_invalid_parameters() -> Result<(), Box<dyn Error>> {
        let content = b"Test content";
        let temp_file = create_test_file(content);
        let path = temp_file.path().to_path_buf();

        let result = shred_file(path.clone(), 0, 1);
        assert!(result.is_err(), "Shredding with 0 passes should fail");

        let result = shred_file(path.clone(), 1, 0);
        assert!(result.is_err(), "Shredding with 0 threads should fail");

        Ok(())
    }

    #[test]
    fn test_shred_file_content_overwritten() -> Result<(), Box<dyn Error>> {
        let content = b"Original content that should be overwritten";
        let temp_file = create_test_file(content);
        let path = temp_file.path().to_path_buf();

        // Perform a single pass of shredding without deleting the file
        {
            let file_path = Arc::new(path.clone());
            let file_size = file_path.metadata()?.len();
            let mut file = fs::OpenOptions::new().write(true).open(&*file_path)?;
            file.seek(io::SeekFrom::Start(0))?;
            let random_data = generate_random_array(file_size);
            file.write_all(&random_data)?;
            file.flush()?;
        }

        // Read the file content after shredding
        let mut file = fs::File::open(&path)?;
        let mut shredded_content = Vec::new();
        file.read_to_end(&mut shredded_content)?;

        assert_ne!(
            shredded_content, content,
            "File content should be overwritten"
        );
        assert!(path.exists(), "File should still exist for this test");

        // Clean up
        fs::remove_file(path)?;

        Ok(())
    }

    fn create_test_directory() -> TempDir {
        TempDir::new().expect("Failed to create temp directory")
    }

    fn create_file(path: &Path, content: &str) {
        let mut file = fs::File::create(path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }

    fn create_directory_structure(root: &Path) {
        fs::create_dir_all(root.join("subdir1")).expect("Failed to create subdir1");
        fs::create_dir_all(root.join("subdir2/subsubdir"))
            .expect("Failed to create subdir2/subsubdir");

        create_file(&root.join("file1.txt"), "Content of file1");
        create_file(&root.join("subdir1/file2.txt"), "Content of file2");
        create_file(&root.join("subdir2/file3.txt"), "Content of file3");
        create_file(
            &root.join("subdir2/subsubdir/file4.txt"),
            "Content of file4",
        );
    }

    #[test]
    fn test_successful_folder_shred() -> Result<(), Box<dyn Error>> {
        let temp_dir = create_test_directory();
        create_directory_structure(temp_dir.path());

        shred_folder(temp_dir.path().to_path_buf(), 3, 2)?;

        assert!(
            !temp_dir.path().exists(),
            "Directory should be deleted after shredding"
        );
        Ok(())
    }

    #[test]
    fn test_shred_empty_folder() -> Result<(), Box<dyn Error>> {
        let temp_dir = create_test_directory();

        shred_folder(temp_dir.path().to_path_buf(), 1, 1)?;

        assert!(
            !temp_dir.path().exists(),
            "Empty directory should be deleted after shredding"
        );
        Ok(())
    }

    #[test]
    fn test_shred_folder_with_only_subdirectories() -> Result<(), Box<dyn Error>> {
        let temp_dir = create_test_directory();
        fs::create_dir_all(temp_dir.path().join("subdir1/subsubdir1"))
            .expect("Failed to create subdirectories");
        fs::create_dir_all(temp_dir.path().join("subdir2"))
            .expect("Failed to create subdirectories");

        shred_folder(temp_dir.path().to_path_buf(), 2, 2)?;

        assert!(
            !temp_dir.path().exists(),
            "Directory with only subdirectories should be deleted after shredding"
        );
        Ok(())
    }

    #[test]
    fn test_shred_folder_with_read_only_file() -> Result<(), Box<dyn Error>> {
        let temp_dir = create_test_directory();
        let readonly_file_path = temp_dir.path().join("readonly.txt");
        create_file(&readonly_file_path, "Read-only content");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&readonly_file_path)?.permissions();
            perms.set_mode(0o444);
            fs::set_permissions(&readonly_file_path, perms)?;
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::FileAttributesExt;
            let mut perms = fs::metadata(&readonly_file_path)?.permissions();
            perms.set_readonly(true);
            fs::set_permissions(&readonly_file_path, perms)?;
        }

        let result = shred_folder(temp_dir.path().to_path_buf(), 1, 1);
        assert!(
            result.is_err(),
            "Shredding folder with read-only file should fail"
        );

        Ok(())
    }
    

}
