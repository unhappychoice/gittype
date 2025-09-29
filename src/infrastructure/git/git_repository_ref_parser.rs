use crate::domain::error::{GitTypeError, Result};
use crate::domain::models::GitRepositoryRef;

pub struct GitRepositoryRefParser;

impl GitRepositoryRefParser {
    pub fn parse(repository_ref: &str) -> Result<GitRepositoryRef> {
        match repository_ref {
            spec if spec.contains('@') => Self::parse_ssh_format(spec),
            spec if spec.starts_with("http") => Self::parse_https_format(spec),
            spec if spec.contains('/') && !spec.contains(' ') => Self::parse_short_format(spec),
            _ => Err(GitTypeError::InvalidRepositoryFormat(format!(
                "Unsupported repository format: {}",
                repository_ref
            ))),
        }
    }

    fn parse_ssh_format(repo_spec: &str) -> Result<GitRepositoryRef> {
        let (host_part, repo_part) = repo_spec.split_once(':').ok_or_else(|| {
            GitTypeError::InvalidRepositoryFormat("Invalid SSH repository format".to_string())
        })?;

        let origin = host_part
            .split('@')
            .nth(1)
            .unwrap_or("github.com")
            .to_string();

        let (owner, name) = repo_part
            .strip_suffix(".git")
            .unwrap_or(repo_part)
            .split_once('/')
            .ok_or_else(|| {
                GitTypeError::InvalidRepositoryFormat("Invalid repository path format".to_string())
            })?;

        Ok(GitRepositoryRef {
            origin,
            owner: owner.to_string(),
            name: name.to_string(),
        })
    }

    fn parse_https_format(repo_spec: &str) -> Result<GitRepositoryRef> {
        let url = repo_spec.strip_suffix(".git").unwrap_or(repo_spec);
        let protocol_end = url.find("://").ok_or_else(|| {
            GitTypeError::InvalidRepositoryFormat("Invalid HTTPS URL format".to_string())
        })?;

        let after_protocol = &url[protocol_end + 3..];
        let parts: Vec<&str> = after_protocol.split('/').collect();

        match parts.as_slice() {
            [origin, owner, name, ..] => Ok(GitRepositoryRef {
                origin: origin.to_string(),
                owner: owner.to_string(),
                name: name.to_string(),
            }),
            _ => Err(GitTypeError::InvalidRepositoryFormat(
                "Invalid HTTPS repository format".to_string(),
            )),
        }
    }

    fn parse_short_format(repo_spec: &str) -> Result<GitRepositoryRef> {
        let (owner, name) = repo_spec.split_once('/').ok_or_else(|| {
            GitTypeError::InvalidRepositoryFormat("Invalid short repository format".to_string())
        })?;

        Ok(GitRepositoryRef {
            origin: "github.com".to_string(),
            owner: owner.to_string(),
            name: name.to_string(),
        })
    }
}
