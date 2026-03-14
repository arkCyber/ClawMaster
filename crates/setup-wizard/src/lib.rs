//! Interactive Setup Wizard for ClawMaster
//!
//! Provides a friendly TUI for first-time setup

mod state;
mod ui;
mod wizard;

#[cfg(test)]
mod tests;

pub use state::ConfigTemplate;
pub use wizard::SetupWizard;

use anyhow::Result;

/// Run the interactive setup wizard
pub async fn run_setup() -> Result<()> {
    let mut wizard = SetupWizard::new()?;
    wizard.run().await
}
