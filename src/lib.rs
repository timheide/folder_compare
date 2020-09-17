/*!
A library to recursively compare files in two folders and return two lists of files: One with new files and one with changed files.

`folder_compare` also takes a list of Strings acting as exclude patterns using `RegexSet`.

Overall the functionality is comparable to a `diff -rq folder1 folder2 -X excludepatterns.pat` on unix like systems

For recognizing changed files, hashing with [`FxHasher`] is used.

[`FxHasher`]: https://github.com/cbreeden/fxhash
*/
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use regex::RegexSet;
use std::hash::Hasher;
use std::fs::File;
use fxhash::FxHasher;
use std::io::Read;

pub struct FolderCompare {
    pub changed_files: Vec<PathBuf>,
    pub new_files: Vec<PathBuf>,
    pub unchanged_files: Vec<PathBuf>,
}

impl FolderCompare {
    /// Instantiates an object of FolderCompare and does the comparison between two `Path` directories and delivers itself consisting of
    /// two lists of `PathBuf` containing changed and new (only existing in first Directory) files.
    /// It takes a `Vec<&str>` as argument for excluding specific substrings in the path (e.g. file extensions like .txt).
    ///
    ///
    /// # Example
    ///
    /// The following code recursively iterates over two directories and returns lists of changed and new files
    ///
    ///```
    /// use std::path::Path;
    /// use folder_compare;
    /// use folder_compare::FolderCompare;
    ///
    ///
    /// let excluded = vec![".doc".to_string(), ".txt".to_string()];
    ///
    /// let result = FolderCompare::new(Path::new("/tmp/a"), Path::new("/tmp/b"), &excluded).unwrap();
    ///
    /// let changed_files = result.changed_files;
    /// let new_files = result.new_files;
    /// let unchanged_files = result.unchanged_files;
    ///```
    ///
    pub fn new(path1: &Path, path2: &Path, excluded: &Vec<String>) -> Result<Self, Error> {

        let mut final_object = FolderCompare {
            changed_files: vec![],
            new_files: vec![],
            unchanged_files: vec![]
        };

        let mut walker = WalkDir::new(path1).into_iter();
        let set = RegexSet::new(excluded)?;

        loop {
            let entry = match walker.next() {
                None => break,
                Some(Err(_)) => continue,
                Some(Ok(entry)) => entry,
            };
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.path_is_symlink() {
                continue;
            }

            if set.matches(entry.path().to_str().unwrap()).matched_any() {
                continue;
            }

            let path_without_prefix = entry.path().strip_prefix(path1)?;
            let file_in_second_path = path2.join(path_without_prefix);
            if !file_in_second_path.is_file() {
                final_object.new_files.push(entry.path().to_path_buf());
                continue;
            }

            let second_file = file_in_second_path.to_path_buf().clone();

            let buffer = &mut vec![];
            File::open(entry.path())?.read_to_end(buffer)?;
            let mut hasher = FxHasher::default();
            hasher.write(buffer);
            let buffer2 = &mut vec![];
            File::open(second_file)?.read_to_end(buffer2)?;
            let mut hasher2 = FxHasher::default();
            hasher2.write(buffer2);

            if hasher.finish() == hasher2.finish() {
                final_object.unchanged_files.push(entry.into_path());
            } else {
                final_object.changed_files.push(entry.into_path());
            }
        }


        Ok(final_object)
    }
}

/// Wrapper for possible errors
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Regex(regex::Error),
    StripPrefix(std::path::StripPrefixError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Error {
        Error::Regex(e)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(e: std::path::StripPrefixError) -> Error {
        Error::StripPrefix(e)
    }
}