use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Welcome to Simple File Renamer!");
    
    let files = fs::read_dir(directory_path)?;

    println!("Files in the directory:");
    for (index, entry) in files.enumerate() {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}. {}", index + 1, file_name.to_string_lossy());
    }

    println!("Enter the number of the file you want to rename:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selected_index: usize = input.trim().parse().expect("Invalid input");

    let selected_file = fs::read_dir(directory_path)?
        .nth(selected_index - 1)
        .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file index"))?;

    let selected_file_path = selected_file?.path();
    
    println!("Enter the new name for the file:");
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name)?;

    let new_name = new_name.trim();
    
    let new_file_path = Path::new(directory_path).join(new_name);

    fs::rename(&selected_file_path, &new_file_path)?;

    println!("File renamed successfully!");

    Ok(())
}

