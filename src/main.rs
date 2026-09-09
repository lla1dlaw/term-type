use color_eyre::eyre::Result;

pub mod app; 
pub mod event;
pub mod update;
pub mod ui; 
pub mod tui;

fn main() -> Result<()> {
    color_eyre::install()?; 

    Ok(())
}
