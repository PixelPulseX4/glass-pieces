#[derive(Debug)]
pub enum DiscoverInputError {
    /// C'est le 1er commit donc y'a pas eu de messages envoyé
    FirstCommit,
    /// Le commit n'est pas traitable car il contient trop de fichiers ou ne contient pas le payload
    IncompatibleCommit,
    /// Le fichier ajouté à un chemin d'accès pas UTF8
    FilePathNotUtf8(gix::bstr::Utf8Error),
    RepoDiscoverError(gix::discover::Error),
    FindExistingReferenceError(gix::reference::find::existing::Error),
    FindExistingObjectError(gix::object::find::existing::Error),
    HeadToCommitError(gix::head::peel::to_commit::Error),
    ObjectCommitError(gix::object::commit::Error),
    DiffTreeToTreeError(gix::repository::diff_tree_to_tree::Error),
}

impl std::fmt::Display for DiscoverInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstCommit => write!(f, "c'est le premier commit"),
            Self::IncompatibleCommit => write!(f, "le commit est incompatible"),
            Self::FilePathNotUtf8(e) => write!(f, "Le chemin d'accès est pas UTF8: {}", e),
            Self::RepoDiscoverError(e) => write!(f, "Repo git introuvable: {}", e),
            Self::FindExistingReferenceError(e) => {
                write!(f, "Échec de recherche d'un référence déjà existante: {}", e)
            }
            Self::FindExistingObjectError(e) => {
                write!(f, "Échec de recherche d'un objet déjà existant: {}", e)
            }
            Self::HeadToCommitError(e) => {
                write!(f, "Échec de conversion du head vers commit: {}", e)
            }
            Self::ObjectCommitError(e) => {
                write!(f, "Échec de résolution du tree d'un commit: {}", e)
            }
            Self::DiffTreeToTreeError(e) => {
                write!(f, "Échec de recherche de différence entre 2 commit: {}", e)
            }
        }
    }
}

impl std::error::Error for DiscoverInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FirstCommit => None,
            Self::IncompatibleCommit => None,
            Self::FilePathNotUtf8(e) => Some(e),
            Self::RepoDiscoverError(e) => Some(e),
            Self::FindExistingReferenceError(e) => Some(e),
            Self::FindExistingObjectError(e) => Some(e),
            Self::HeadToCommitError(e) => Some(e),
            Self::ObjectCommitError(e) => Some(e),
            Self::DiffTreeToTreeError(e) => Some(e),
        }
    }
}

impl From<gix::discover::Error> for DiscoverInputError {
    fn from(value: gix::discover::Error) -> Self {
        Self::RepoDiscoverError(value)
    }
}

impl From<gix::object::find::existing::Error> for DiscoverInputError {
    fn from(value: gix::object::find::existing::Error) -> Self {
        Self::FindExistingObjectError(value)
    }
}

impl From<gix::reference::find::existing::Error> for DiscoverInputError {
    fn from(value: gix::reference::find::existing::Error) -> Self {
        Self::FindExistingReferenceError(value)
    }
}

impl From<gix::head::peel::to_commit::Error> for DiscoverInputError {
    fn from(value: gix::head::peel::to_commit::Error) -> Self {
        Self::HeadToCommitError(value)
    }
}

impl From<gix::object::commit::Error> for DiscoverInputError {
    fn from(value: gix::object::commit::Error) -> Self {
        Self::ObjectCommitError(value)
    }
}

impl From<gix::repository::diff_tree_to_tree::Error> for DiscoverInputError {
    fn from(value: gix::repository::diff_tree_to_tree::Error) -> Self {
        Self::DiffTreeToTreeError(value)
    }
}
