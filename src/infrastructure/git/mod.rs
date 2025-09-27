pub mod local;
pub mod remote;
pub mod git_repository_ref_parser;

pub use local::LocalGitRepositoryClient;
pub use remote::RemoteGitRepositoryClient;