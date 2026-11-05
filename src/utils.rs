use crate::cli::CreateOptions;
use crate::app::RunError;

use std::path::Path;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Write};
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

