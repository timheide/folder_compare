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
use std::io::{Read};

/// Does the comparison between two `Path` directories and delivers two lists of `PathBuf` containing changed and new (only existing in first Directory) files.
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
///
///
/// let excluded = vec![".doc", ".txt"];
/// let (changed_files, new_files) = folder_compare::compare(Path::new("/tmp/a"), Path::new("/tmp/b"), &excluded).unwrap();
///```
///
pub fn compare(path1: &Path, path2: &Path, excluded: &Vec<&str>) -> Result<(Vec<PathBuf>, Vec<PathBuf>), Error> {
    let mut changed_files: Vec<PathBuf> = Vec::new();
    let mut new_files: Vec<PathBuf> = Vec::new();
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
            new_files.push(entry.path().to_path_buf());
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
            continue;
        }
        changed_files.push(entry.into_path());
    }


    Ok((changed_files, new_files))

}

/// Wrapper for possible errors
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Regex(regex::Error),
    StripPrefix(std::path::StripPrefixError)
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