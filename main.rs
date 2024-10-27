use std::{fs, io};
use rand::seq::SliceRandom; // Import for random shuffling

fn process_files_in_category(category: &str, base_path: &str) -> io::Result<()> {
    let category_path = format!("{}/{}", base_path, category);
    let mut files = vec![];

    // Collect all files in the category and its subdirectories
    for entry in fs::read_dir(&category_path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.path());
        }
    }

    // Shuffle files using rand
    let mut rng = rand::thread_rng();
    files.shuffle(&mut rng);

    Ok(())
}

fn main() -> io::Result<()> {
    let categories = ["takeoff", "land", "right", "left", "forward", "backward"];
    let base_path = "data";

    for category in &categories {
        process_files_in_category(category, base_path)?;
    }

    Ok(())
}
