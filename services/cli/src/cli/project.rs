use clap::{
    Parser,
    Subcommand,
};

use super::CliBackend;

#[derive(Subcommand, Debug)]
pub enum CliProject {
    // Create a project
    #[clap(alias = "add")]
    Create(CliProjectCreate),
    // List projects
    #[clap(alias = "ls")]
    List(CliProjectList),
    // View a project
    View(CliProjectView),
}

#[derive(Parser, Debug)]
pub struct CliProjectCreate {
    /// Project name
    pub name: String,

    /// Project slug
    #[clap(long)]
    pub slug: Option<String>,

    /// Project description
    #[clap(long)]
    pub description: Option<String>,

    /// Project URL
    #[clap(long)]
    pub url: Option<String>,

    /// Set as default project
    #[clap(long)]
    pub default: bool,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliProjectList {
    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliProjectView {
    /// Project slug or UUID
    pub project: String,

    #[clap(flatten)]
    pub backend: CliBackend,
}