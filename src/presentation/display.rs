use crate::domain::SearchResult;
use colored::Colorize;
use comfy_table::{ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use indicatif::{ProgressBar, ProgressStyle};

pub struct DisplayManager;

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayManager {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn show_search_results(&self, results: &SearchResult) {
        if results.extensions.is_empty() {
            println!("{}", "No extensions found.".yellow());
            return;
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec![
                "Name".bold().to_string(),
                "Downloads".bold().to_string(),
                "Publisher".bold().to_string(),
                "ID".bold().to_string(),
            ]);

        for ext in &results.extensions {
            table.add_row(vec![
                ext.display_name.clone(),
                format_downloads(ext.downloads),
                ext.publisher.clone(),
                ext.unique_identifier(),
            ]);
        }

        println!(
            "\n{}",
            format!("Found {} extensions:", results.total_count).green()
        );
        println!("{table}");
    }

    /// Shows a progress spinner for installation
    ///
    /// # Panics
    ///
    /// Panics if the progress bar template cannot be parsed
    #[must_use]
    pub fn show_installing(&self, extension_id: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message(format!("Installing {extension_id}..."));
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        pb
    }

    pub fn show_success(&self, message: &str) {
        println!("{} {}", "✓".green(), message.green());
    }

    pub fn show_error(&self, error: &str) {
        eprintln!("{} {}", "✗".red(), error.red());
    }
}

#[allow(clippy::cast_precision_loss)]
fn format_downloads(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_downloads() {
        assert_eq!(format_downloads(500), "500");
        assert_eq!(format_downloads(1_500), "1.5K");
        assert_eq!(format_downloads(1_500_000), "1.5M");
    }
}
