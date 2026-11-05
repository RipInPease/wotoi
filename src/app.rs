use crate::cli::{Args, Mode, ParseError, CreateOptions};
use crate::utils;

use std::fs;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};


pub struct SkillIssue {
    name: String,
    tags: Vec<String>,
    description: String,
    priority: u8,
}


pub const ISSUES_DIR: &'static str = "skill_issues";

#[derive(Debug)]
pub enum RunError {
    IOError(IOError),
    IssueAlreadyExists,
    IssueDoesNotExist(String),
}

impl From<IOError> for RunError {
    fn from(value: IOError) -> Self {
        Self::IOError(value)
    }
}

pub fn run_app(args: Args) {
    let res = match args.mode {
        Mode::Create(create_options) => create_issue(create_options),
        Mode::Remote(issue_name) => remove_issue(issue_name),
        _ => todo!()
    };

    if res.is_err() {
        print_run_error(res.unwrap_err());
    }
}

fn create_issue(create_options: CreateOptions) -> Result<(), RunError> {
    utils::dir_exists(ISSUES_DIR)?;
    
    let issue_path = ISSUES_DIR.to_string() + "/" + &create_options.name + ".md";
    let mut issue = utils::create_issue_file(issue_path)?;
    utils::write_create_options_to_issue(create_options, &mut issue)?;
    
    Ok(())
}

fn remove_issue(issue_name: String) -> Result<(), RunError> {
    let path = ISSUES_DIR.to_string() + "/" + &issue_name + ".md";

    if let Err(e) = fs::remove_file(path) {
        if e.kind() == IOErrorKind::NotFound {
            return Err(RunError::IssueDoesNotExist(issue_name))
        }

        return Err(e)?
    }

    Ok(())
}

pub fn print_parse_error(error: ParseError) {
    println!("Parse error: {error:#?}");
}

fn print_run_error(error: RunError) {
    use RunError::*;

    match error {
        IOError(_) => {
            print!("Failed to read/write to file/folder in ./skill_issues/, please check permissions")
        },
        IssueAlreadyExists => {
            println!("It seems you tried to create an issue that already exists, this is a major skill issue on your part");
        },
        IssueDoesNotExist(issue_name) => {
            println!("It seems you tried to change the issue [{issue_name}], which does not exist, you silly goose")
        }
    }
}