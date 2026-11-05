use std::fs;
use std::path::{Path, PathBuf};

use crate::{Result, WyagError};

pub struct Repository {
    pub worktree: PathBuf,
    pub git_dir: PathBuf,
}

impl Repository {
    pub fn init(path: &Path) -> Result<Self> {
        let git_dir = path.join(".git");
        if git_dir.exists() {
            return Err(WyagError::RepositoryExists(git_dir));
        }

        for dir in ["objects", "refs/heads", "refs/tags", "branches"] {
            fs::create_dir_all(git_dir.join(dir))?;
        }

        fs::write(git_dir.join("HEAD"), "ref: refs/heads/master\n")?;

        fs::write(
            git_dir.join("description"),
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )?;

        fs::write(
            git_dir.join("config"),
            "[core]\n\trepositoryformatversion = 0\n\tfilemode = false\n\tbare = false\n",
        )?;

        Ok(Self {
            worktree: path.to_path_buf(),
            git_dir,
        })
    }

    pub fn find(start: &Path) -> Result<Self> {
        let start = std::path::absolute(start)?;
        for dir in start.ancestors() {
            let git_dir = dir.join(".git");
            if git_dir.is_dir() {
                return Ok(Self {
                    worktree: dir.to_path_buf(),
                    git_dir,
                });
            }
        }
        Err(WyagError::NotARepository(start))
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn find_from_root_and_subdirectory() {
        let dir = TempDir::new().unwrap();
        Repository::init(dir.path()).unwrap();
        let nested = dir.path().join("a").join("b");
        fs::create_dir_all(&nested).unwrap();

        for start in [dir.path(), nested.as_path()] {
            let repo = Repository::find(start).unwrap();
            assert_eq!(repo.git_dir, dir.path().join(".git"));
        }
    }

    #[test]
    fn find_outside_repo_fails() {
        let dir = TempDir::new().unwrap();
        assert!(matches!(
            Repository::find(dir.path()),
            Err(WyagError::NotARepository(_))
        ));
    }

    #[test]
    fn init_refuses_existing_repo() {
        let dir = TempDir::new().unwrap();
        Repository::init(dir.path()).unwrap();
        assert!(matches!(
            Repository::init(dir.path()),
            Err(WyagError::RepositoryExists(path)) if path == dir.path().join(".git")
        ));
    }
}
