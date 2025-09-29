#[derive(Debug, Clone)]
pub struct GitRepositoryRef {
    pub origin: String,
    pub owner: String,
    pub name: String,
}

impl GitRepositoryRef {
    pub fn http_url(&self) -> String {
        format!("https://{}/{}/{}.git", self.origin, self.owner, self.name)
    }
}
