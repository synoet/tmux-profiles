use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // launch a single profile
    #[command(arg_required_else_help = true, about = "launch a profile")]
    Launch {
        #[arg(
            value_name = "PROFILE_NAME",
            required = true,
            help = "name of the profile you want to launch"
        )]
        name: String,
        #[arg(
            value_name = "FORCE",
            required = false,
            help ="will re-create the session if it already exists",
            default_value = None,
        )]
        force: Option<bool>,
    },
    // launch all profiles in a group
    #[command(arg_required_else_help = true, about = "launch a group of profiles")]
    Group {
        #[arg(
            value_name = "GROUP_NAME",
            required = true,
            help = "name of the group of profiles you want to launch"
        )]
        name: String,
        #[arg(
            value_name = "FORCE",
            required = false,
            help ="will re-create the session if it already exists",
            default_value = None,
        )]
        force: Option<bool>,
    },
    // Lists all available profiles
    #[command(about = "list groups & profiles")]
    List,
}
