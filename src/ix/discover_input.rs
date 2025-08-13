mod error;

pub use error::DiscoverInputError;
use gix::bstr::ByteSlice;

fn get_single_file_path_from_changes(
    changes: &[gix::diff::tree_with_rewrites::Change],
) -> Result<String, DiscoverInputError> {
    let mut file_path: Option<&str> = None;

    for change in changes {
        let path = change
            .location()
            .to_str()
            .map_err(|e| DiscoverInputError::FilePathNotUtf8(e))?;

        if file_path.as_ref().is_some_and(|s| *s == path) {
            return Err(DiscoverInputError::CommitWithTooMuchFileChanges);
        }

        file_path = Some(path)
    }

    file_path
        .map(|s| s.to_owned())
        .ok_or(DiscoverInputError::CommitWithoutFileChanges)
}

pub fn discover_input() -> Result<String, DiscoverInputError> {
    let repo = gix::discover(".")?;
    let mut head = repo.head()?;
    let curr_commit = head.peel_to_commit_in_place()?;

    let parent_id = curr_commit
        .parent_ids()
        .next()
        .ok_or(DiscoverInputError::FirstCommit)?;

    let parent_commit = repo.find_object(parent_id)?.into_commit();

    let changes = repo.diff_tree_to_tree(
        Some(&parent_commit.tree()?),
        Some(&curr_commit.tree()?),
        None,
    )?;

    let path = get_single_file_path_from_changes(&changes)?;

    if !path.starts_with("guette-guette/") {
        return Err(DiscoverInputError::CommitChangeIsIncompatible(
            path.to_owned(),
        ));
    }

    Ok(path.to_string())
}
