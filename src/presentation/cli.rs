use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vsix")]
#[command(author, version, about = "Downloads and installs .vsix extensions into Visual Studio Code and Cursor", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, help = "Custom marketplace URL")]
    pub marketplace: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Search for extensions in the marketplace")]
    Search {
        #[arg(help = "Search query")]
        query: String,

        #[arg(
            short,
            long,
            help = "Sort results by field (name, downloads, publisher)",
            default_value = "downloads"
        )]
        sort: String,

        #[arg(short, long, help = "Reverse sort order")]
        reverse: bool,

        #[arg(short, long, help = "Limit number of results", default_value = "20")]
        limit: usize,
    },

    #[command(about = "Install an extension")]
    Install {
        #[arg(help = "Extension ID (format: publisher.name)")]
        id: String,

        #[arg(
            long,
            help = "Install to VSCode",
            default_value = "true",
            conflicts_with = "cursor"
        )]
        vscode: bool,

        #[arg(long, help = "Install to Cursor")]
        cursor: bool,
    },
}
