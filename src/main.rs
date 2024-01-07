use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

fn search_and_replace(file_content: &mut String, search_text: &str, replace_text: &str) {
    *file_content = file_content.replace(search_text, replace_text);
}

fn main() {
    println!("Welcome to the File Operations Program!");

    // Get the folder name from the user
    println!("Please enter the path of the folder you want to operate on:");
    let mut folder_name = String::new();
    io::stdin().read_line(&mut folder_name).expect("Failed to read the folder name.");
    let folder_name = folder_name.trim();

    // List files in the folder
    match fs::read_dir(folder_name) {
        Ok(entries) => {
            let mut file_list = Vec::new();

            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let file_name = dir_entry.file_name();
                        println!("File name: {:?}", file_name);
                        file_list.push(dir_entry);
                    }
                    Err(e) => eprintln!("An error occurred while reading the file: {}", e),
                }
            }

            // Select a file after choosing the folder
            println!("Please enter the name of the file you want to open:");
            let mut selected_file_name = String::new();
            io::stdin().read_line(&mut selected_file_name).expect("Failed to read the file name.");
            let selected_file_name = selected_file_name.trim();

            // Find and open the selected file
            if let Some(selected_file) = file_list.iter().find(|&entry| {
                entry.file_name().to_string_lossy() == selected_file_name
            }) {
                let mut file_content = String::new();

                match File::open(selected_file.path()) {
                    Ok(mut file) => {
                        match file.read_to_string(&mut file_content) {
                            Ok(_) => println!("File content:\n{}", file_content),
                            Err(e) => eprintln!("An error occurred while reading the file: {}", e),
                        }
                    }
                    Err(e) => eprintln!("An error occurred while opening the file: {}", e),
                }

                // Ask the user if they want to reset the content of the file
                println!("Do you want to reset the content of the file? (Yes/No)");
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to read the response.");
                let response = response.trim().to_lowercase();

                if response == "yes" {
                    // Reset the file
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(selected_file.path())
                        .expect("Failed to open the file.");

                    // Write success message to the reset file
                    if let Err(e) = write!(file, "File has been reset.") {
                        eprintln!("An error occurred while writing to the file: {}", e);
                    } else {
                        println!("The content of the file has been successfully reset.");
                    }
                }

                // Ask the user for a word or phrase
                println!("Please enter the word or phrase you want to search for in the file:");
                let mut search_word = String::new();
                io::stdin().read_line(&mut search_word).expect("Failed to read the word.");
                let search_word = search_word.trim();

                // Count the occurrences of the word in the file
                let word_count = file_content.split_whitespace().filter(|&word| word == search_word).count();
                println!("The word '{}' appears {} times in the file.", search_word, word_count);

                // Ask the user if they want to replace the word
                println!("Do you want to replace the word '{}' with another word or text? (Yes/No)", search_word);
                let mut replace_response = String::new();
                io::stdin().read_line(&mut replace_response).expect("Failed to read the response.");
                let replace_response = replace_response.trim().to_lowercase();

                if replace_response == "yes" {
                    // Ask the user for the new text
                    println!("Please enter what you want to replace the word '{}' with:", search_word);
                    let mut replace_text = String::new();
                    io::stdin().read_line(&mut replace_text).expect("Failed to read the text.");

                    // Replace the word in the file
                    search_and_replace(&mut file_content, search_word, replace_text.trim());

                    // Open the file and write the modified text to the file
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(selected_file.path())
                        .expect("Failed to open the file.");

                    if let Err(e) = write!(file, "{}", file_content.trim()) {
                        eprintln!("An error occurred while writing to the file: {}", e);
                    } else {
                        println!("The word '{}' in the file has been successfully replaced.", search_word);
                    }
                }
            } else {
                eprintln!("The specified file could not be found.");
            }
        }
        Err(e) => eprintln!("An error occurred while reading the folder: {}", e),
    }
}
