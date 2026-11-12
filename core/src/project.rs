pub struct ProjectInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: &'static str,
    pub description: &'static str,
}

pub const PROJECT: ProjectInfo = ProjectInfo {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    authors: env!("CARGO_PKG_AUTHORS"),
    description: env!("CARGO_PKG_DESCRIPTION"),
};
