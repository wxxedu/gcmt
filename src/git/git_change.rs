use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitChangeStatus {
    Staged,
    Unstaged,
    Untracked,
}

impl Display for GitChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            GitChangeStatus::Staged => "Staged",
            GitChangeStatus::Unstaged => "Unstaged",
            GitChangeStatus::Untracked => "Untracked",
        };
        write!(f, "{}", status_str)
    }
}

impl GitChangeStatus {
    pub fn get_status_code(&self) -> usize {
        match self {
            GitChangeStatus::Staged => 0,
            GitChangeStatus::Unstaged => 1,
            GitChangeStatus::Untracked => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitChange {
    pub path: String,
    pub status: GitChangeStatus,
}

impl Display for GitChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t: {}", self.status, self.path)
    }
}

impl PartialOrd for GitChange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.status == other.status {
            return Some(self.path.cmp(&other.path));
        }
        Some(
            self.status
                .get_status_code()
                .cmp(&other.status.get_status_code()),
        )
    }
}

impl GitChange {
    pub fn new(path: String, status: GitChangeStatus) -> Self {
        Self { path, status }
    }

    pub fn stage(&mut self) {
        let output = super::git::Git::new_git_command()
            .arg("add")
            .arg(&self.path)
            .output()
            .expect("Failed to execute git add");
        self.status = GitChangeStatus::Staged;
        if !output.status.success() {
            panic!("Failed to execute git add: {:?}", output);
        }
    }

    pub fn unstage(&mut self) {
        let output = super::git::Git::new_git_command()
            .arg("reset")
            .arg(&self.path)
            .output()
            .expect("Failed to execute git reset");
        self.status = GitChangeStatus::Unstaged;
        if !output.status.success() {
            panic!("Failed to execute git reset");
        }
    }
}

pub trait GitChanges {
    fn has_staged_changes(&self) -> bool;
    fn has_unstaged_changes(&self) -> bool;
    fn has_untracked_changes(&self) -> bool;
    fn has_changes(&self) -> bool;
}

impl GitChanges for Vec<GitChange> {
    fn has_staged_changes(&self) -> bool {
        self.iter()
            .any(|change| change.status == GitChangeStatus::Staged)
    }

    fn has_unstaged_changes(&self) -> bool {
        self.iter()
            .any(|change| change.status == GitChangeStatus::Unstaged)
    }

    fn has_untracked_changes(&self) -> bool {
        self.iter()
            .any(|change| change.status == GitChangeStatus::Untracked)
    }

    fn has_changes(&self) -> bool {
        !self.is_empty()
    }
}
