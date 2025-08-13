mod error;

pub use error::DiscoverInputError;
use gix::bstr::ByteSlice;

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
        Some(&parent_commit.tree()?),
        None,
    )?;
    if changes.len() > 1 {
        return Err(DiscoverInputError::IncompatibleCommit);
    }
    let change = changes
        .into_iter()
        .next()
        .ok_or(DiscoverInputError::IncompatibleCommit)?;

    let path = change
        .location()
        .to_str()
        .map_err(|e| DiscoverInputError::FilePathNotUtf8(e))?;

    let is_new = !change.entry_mode().is_no_tree();
    if !path.starts_with("guette-guette/") || !is_new {
        return Err(DiscoverInputError::IncompatibleCommit);
    }

    Ok(path.to_string())
}
