use clap::Parser;
use vsix_install::application::ApplicationService;
use vsix_install::presentation::{Cli, Commands, DisplayManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let service = ApplicationService::new();
    let display = DisplayManager::new();
    
    match cli.command {
        Commands::Search { query } => {
            match service.search_extensions(&query, cli.marketplace.as_deref()).await {
                Ok(results) => {
                    display.show_search_results(&results);
                }
                Err(e) => {
                    display.show_error(&format!("Search failed: {}", e));
                    std::process::exit(1);
                }
            }
        }
        Commands::Install { id, cursor, .. } => {
            let pb = display.show_installing(&id);
            
            match service.install_extension(&id, cursor, cli.marketplace.as_deref()).await {
                Ok(_) => {
                    pb.finish_and_clear();
                    let target = if cursor { "Cursor" } else { "VSCode" };
                    display.show_success(&format!("Successfully installed {} to {}", id, target));
                }
                Err(e) => {
                    pb.finish_and_clear();
                    display.show_error(&format!("Installation failed: {}", e));
                    std::process::exit(1);
                }
            }
        }
    }
    
    Ok(())
}