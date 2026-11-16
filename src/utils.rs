use crate::cli::CreateOptions;
use crate::app::{RunError, SkillIssue};

use std::path::Path;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Write, Read, BufReader};
use std::fs::{self, File};


/// Makes sure a directory exists. 
/// Will make one if it does not and will return [`Err`] if not
pub fn dir_exists<P: AsRef<Path>>(path: P) -> Result<(), IOError> {
    if let Err(e) = fs::create_dir(path) && e.kind() != IOErrorKind::AlreadyExists {
        Err(e)
    } else {
        Ok(())
    }
}

/// Creates a new issue file
pub fn create_issue_file<P: AsRef<Path>>(path: P) -> Result<File, RunError> {
    match fs::exists(&path)? {
        true => Err(RunError::IssueAlreadyExists),
        false => Ok(fs::File::create(path)?)
    }
}

/// Writes the basic create option to a file
pub fn write_create_options_to_issue(create_options: CreateOptions, file: &mut File) -> Result<(), RunError> {
    let mut buf = String::with_capacity(256);

    buf.push_str("# ");
    buf.push_str(&create_options.name);

    buf.push_str("\n\n## Description: ");
    buf.push_str(&format!("{}", create_options.priority));
    
    if let Some(tags) = create_options.tags {
        buf.push_str("\n\n## Tags:\n");

        for tag in tags {
            buf.push_str("- ");
            buf.push_str(&tag);
            buf.push('\n');
        }
    }

    if let Some(description) = create_options.description {
        buf.push_str("\n### Description:\n");
        buf.push_str(&description);
    }

    let mut buf = buf.into_bytes();
    file.write_all(&mut buf[..])?;

    Ok(())
}

/// Reads the next char from a reader.
/// If it encounters an unknown char, it returns � 
fn read_char<R: Read>(reader: &mut R) -> Result<char, IOError> {
    let mut buf = [0; 4];
    
    reader.read_exact(&mut buf[0..1])?;

    let first_byte = buf[0];

    let remaining_bytes = match first_byte {
        0b00000000..=0b01111111 => return Ok(
                char::try_from(first_byte).unwrap_or(char::REPLACEMENT_CHARACTER)
            ),
        0b11000000..=0b11011111 => 1,
        0b11100000..=0b11101111 => 2,
        0b11110000..=0b11110111 => 3,
        _ => return Ok(char::REPLACEMENT_CHARACTER)
    };

    println!("Total bytes of char: {}", remaining_bytes + 1);

    reader.read_exact(&mut buf[1..remaining_bytes+1])?;
    
    match str::from_utf8(&buf[0..remaining_bytes+1]) {
        Ok(s) => Ok(s.chars().next().unwrap_or(char::REPLACEMENT_CHARACTER)),
        Err(_) => Ok(char::REPLACEMENT_CHARACTER)
    }
}

pub fn get_issue_paths<P: AsRef<Path>>(dir: P) -> Result<Vec<String>, RunError> {
    let mut res = Vec::with_capacity(16);

    let read_dir = fs::read_dir(dir)?;

    for entry in read_dir {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            continue;
        }

        let file_path = match entry.path().into_string() {
            Ok(s) => s,
            Err(_) => continue,
        };

        if file_path.ends_with(".md") {
            res.push(file_path);
        }
    }

    Ok(res)
}

pub fn read_issue<P: AsRef<Path>>(path: P) -> Result<SkillIssue, RunError> {
    let file = fs::read_to_string(path)?;

    let name_pos = file.find("# ");

    

    todo!()
}