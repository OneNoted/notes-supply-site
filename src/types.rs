#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: u32,
    pub forks: u32,
    pub topics: Vec<String>,
    pub html_url: String,
    pub homepage: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ProjectListItem {
    pub name: String,
    pub slug: String,
    pub language: String,
    pub stars: u32,
    pub updated: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GithubStats {
    pub repos: usize,
    pub stars: u32,
    pub forks: u32,
    pub contributions: u32,
    pub languages: Vec<LangStat>,
    pub latest_project: String,
    pub latest_updated: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct LangStat {
    pub name: String,
    pub percent: f64,
}

