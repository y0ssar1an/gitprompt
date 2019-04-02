//! Print a colorized zsh prompt with the current git branch.

use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;

fn main() -> io::Result<()> {
    let wd = env::current_dir()?;
    let branch = match current_branch(wd.as_path()) {
        Some(b) => b,
        None => return Ok(()), // current dir is not a git repo
    };

    if is_dirty() {
        println!(" %F{{blue}}(%F{{red}}{}%F{{blue}})%f💩", branch);
    } else {
        println!(" %F{{blue}}(%F{{red}}{}%F{{blue}})%f", branch);
    };

    Ok(())
}

/// Return true if the working tree is dirty. "Dirty" means that there are
/// modifications that haven't been committed yet.
fn is_dirty() -> bool {
    let output = match process::Command::new("git")
        .arg("status")
        .arg("--short")
        .output()
    {
        Ok(out) => out,
        Err(_) => return false,
    };

    !output.stdout.is_empty()
}

/// Return the name of the current branch. If we're in a directory that isn't
/// inside a git repo, return `None`.
fn current_branch(wd: &path::Path) -> Option<String> {
    if inside_dotgit_dir(wd) {
        // Print ".git" instead of the branch name.
        return Some(".git".to_string());
    }

    // Find the path to the .git/HEAD file.
    let head_path = match find_head(wd) {
        Some(p) => p,
        None => return None,
    };

    // Read .git/HEAD and extract the branch name.
    read_head(head_path.as_path()).ok()
}

/// Return true if we're inside the hidden .git/ directory in a repo.
fn inside_dotgit_dir(wd: &path::Path) -> bool {
    for path_component in wd {
        if path_component == ".git" {
            return true;
        }
    }

    false
}

/// Return the absolute path to the .git/HEAD file, which contains the name of
/// the current branch. If the current working directory isn't in a git repo, it
/// will return None.
fn find_head(dir: &path::Path) -> Option<path::PathBuf> {
    // Iterate through all the parent directories and see if $DIR/.git/HEAD is
    // a file that exists.
    //   /home/me/projects/foo/src/bar/.git/HEAD ??? -> doesn't exist
    //   /home/me/projects/foo/src/.git/HEAD ???     -> doesn't exist
    //   /home/me/projects/foo/.git/HEAD ???         -> found it!
    for d in dir.ancestors() {
        let p = d.join(".git/HEAD");
        if p.is_file() {
            return Some(p);
        }
    }

    None
}

/// Read the .git/HEAD file and extract the name of the current branch.
fn read_head(path_to_head: &path::Path) -> io::Result<String> {
    let mut s = fs::read_to_string(path_to_head)?;
    s = s.trim().trim_start_matches("ref: refs/heads/").to_string();
    Ok(s)
}
