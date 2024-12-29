use anyhow::Result;
use app_state::AppState;
use views::todo::TodoSet;
use widgetui::App;

mod app_state;
mod models;
mod utils;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    let app_state = AppState::new().await?;

    App::new(100)?.states(app_state).sets(TodoSet).run()?;
    return Ok(());
}
