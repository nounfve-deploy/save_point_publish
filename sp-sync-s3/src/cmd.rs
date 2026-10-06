/// sync cmd line for save point
#[derive(clap::Parser)]
pub struct Cmd {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand, Clone)]
pub enum Action {
    /// list storage
    List,
    Pull {
        key: String,
    },
    Sync {
        key: String,
    },
}
