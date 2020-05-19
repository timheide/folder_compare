folder_compare
=========================
A library to recursively compare files in two folders and return two lists of files: One with new files and one with changed files.

`folder_compare` also takes a list of Strings acting as exclude patterns using `RegexSet`.

Overall the functionality is comparable to a `diff -rq folder1 folder2 -X excludepatterns.pat` on unix like systems

For recognizing changed files, hashing with [`FxHasher`] is used.

Licensed under Apache-2.0

### Usage

To use this crate, add `walkdir` as a dependency to your project's
`Cargo.toml`:

```toml
[dependencies]
folder_compare = "0.1"
```

 # Example

 The following code recursively iterates over two directories and returns lists of changed and new files

```
use walkdir::WalkDir;
use std::path::Path;
use folder_compare;

let excluded = vec![".doc", ".txt"];
let (changed_files, new_files) = folder_compare::compare(Path::new("/tmp/a"), Path::new("/tmp/b"), &excluded).unwrap();
```