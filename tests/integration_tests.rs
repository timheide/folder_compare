use folder_compare;
use std::path::PathBuf;
use std::{env, fs};
use std::fs::{create_dir, remove_dir_all};
use std::io::Error;

#[test]
fn one_changed_one_new_one_ignored() {
    let dirs = prepare_environment().unwrap();
    let excluded = vec![".doc", ".txt"];
    let (a, b) = folder_compare::compare(dirs.0.as_path(), dirs.1.as_path(), &excluded).unwrap();

    remove_dir_all(dirs.1.parent().unwrap()).unwrap();
    assert_eq!((a.len(), b.len()), (1, 1));
}

fn prepare_environment() -> Result<(PathBuf, PathBuf), Error> {
    let mut base_dir = env::temp_dir();
    base_dir.push("compare");
    create_dir(&base_dir)?;

    let mut dir_a = base_dir.clone();
    dir_a.push("a");
    create_dir(&dir_a)?;
    //one new
    dir_a.push("test.abc");
    fs::write(&dir_a, "Test")?;
    dir_a.pop();
    //one changed
    dir_a.push("test.xls");
    fs::write(&dir_a, "Test")?;
    dir_a.pop();
    //one excluded
    dir_a.push("test.txt");
    fs::write(&dir_a, "Test")?;
    dir_a.pop();

    let mut dir_b = base_dir.clone();
    dir_b.push("b");
    create_dir(&dir_b)?;
    dir_b.push("test.xls");
    fs::write(&dir_b, "Test2")?;
    dir_b.pop();
    Ok((dir_a, dir_b))
}