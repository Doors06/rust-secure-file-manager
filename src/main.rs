// ============== File Management utility in Rust  ================
// This program allows users to securely manage files by backing them up, 
//restoring from backups, and deleting files.

// Favian Puertas - Evan Boantua - John Moncada ** July 2025

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use chrono::Local; // For timestamping log entries

// Get current time in formatted string
fn get_current_time() -> String {
    let now = Local::now();
    format!("[{}] ", now.format("%Y-%m-%d %H:%M:%S"))
}

// Logs an action securely to logfile.txt
fn log_action(action: &str) {
    let log_path = "logfile.txt";
    if let Ok(mut log_file) = OpenOptions::new().create(true).append(true).open(log_path) {

        // IMPORTANT!! - Using fixed format, prevents format string vulnerabilities
        if let Err(e) = writeln!(log_file, "{}{}", get_current_time(), action) {
            eprintln!("Failed to write to log file: {}", e);
        }
    } else {
        eprintln!("Failed to open log file.");
    }
}

// Backup file by creating a copy with .bak extension
fn backup_file(filename: &str) {
    let path = Path::new(filename);

    // IMPORTANT!! - Check if the file exists before proceeding
    if !path.exists() {
        eprintln!("Error: File does not exist.");
        return;
    }

    let backup_name = format!("{}.bak", filename);

    // IMPORTANT!! - Use std::fs::copy for safe file copy
    match fs::copy(path, &backup_name) {
        Ok(_) => {
            println!("Backup created: {}", backup_name);
            log_action(&format!("Performed backup on file: {}", filename));
        }
        Err(e) => {
            eprintln!("Backup failed: {}", e);
        }
    }
}

// Restore file from .bak version
fn restore_file(filename: &str) {
    let backup_name = format!("{}.bak", filename);
    let backup_path = Path::new(&backup_name);

    // IMPORTANT!! - Check if backup exists
    if !backup_path.exists() {
        eprintln!("Backup file not found.");
        return;
    }

    match fs::copy(&backup_name, filename) {
        Ok(_) => {
            println!("File restored from: {}", backup_name);
            log_action(&format!("Performed restore on file: {}", filename));
        }
        Err(e) => {
            eprintln!("Restore failed: {}", e);
        }
    }
}

// Delete file with confirmation
fn delete_file(filename: &str) {
    let path = Path::new(filename);

    // Check existence
    if !path.exists() {
        eprintln!("File does not exist.");
        return;
    }

    println!("Are you sure you want to delete \"{}\"? (yes/no): ", filename);
    // Create a mutable string buffer to hold the user's typed confirmation; 
    // must be mutable to allow reading input into it
    let mut confirm = String::new();
    if let Err(_) = io::stdin().read_line(&mut confirm) {
        eprintln!("Failed to read input.");
        return;
    }

    if confirm.trim() == "yes" {
        // IMPORTANT!! - Safe file deletion
        match fs::remove_file(path) {
            Ok(_) => {
                println!("File deleted.");
                log_action(&format!("Deleted file: {}", filename));
            }
            Err(e) => {
                eprintln!("Failed to delete file: {}", e);
            }
        }
    } else {
        println!("Deletion cancelled.");
    }
}

// Validate filename to avoid path traversal and invalid input
fn is_valid_filename(filename: &str) -> bool {
    // IMPORTANT!! - Prevent path traversal attacks and empty input
    !filename.trim().is_empty()
        && !filename.contains("..")
        && !filename.contains('/')
        && !filename.contains('\\')
}

// Main program entry
fn main() {
    let mut filename = String::new();
    println!("Please enter your file name: ");
    // Try to read the user's input into the 'filename' buffer.
    // If reading fails (e.g., input/output error), handle it by matching the error with 'if let Err'
    if let Err(_) = io::stdin().read_line(&mut filename) {
        eprintln!("Failed to read filename.");
        return;
    }
    let filename = filename.trim();

    if !is_valid_filename(filename) {
        eprintln!("Invalid file name.");
        return;
    }

    // Prompt user for command
    let mut command = String::new();
    println!("Please enter your command (backup, restore, delete): ");
    if let Err(_) = io::stdin().read_line(&mut command) {
        eprintln!("Failed to read command.");
        return;
    }

    // takes action depending on the command
    match command.trim() {
        "backup" => backup_file(filename),
        "restore" => restore_file(filename),
        "delete" => delete_file(filename),
        _ => eprintln!("Unknown command."),
    }
}

