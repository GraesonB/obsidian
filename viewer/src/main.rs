// obsidian/viewer/src/main.rs
use anyhow::Result;
use log::info;
use obsidian::app::{run_application, AppConfig, Application, Run}; //we can use this path because of our toml file
 // The above works because toml says obsidian steps backwards into the main directory
 // we then declare that app is module that makes everything in the obsidian_app available through the mod
 // in combination with pub use obsidian_app::* which gets us obsidian::app
 // Then finally, inside the obsidian_app crate pub self::app::* is declared, so anything
 // inside that crate's app.rs is exposed through obsidian::app
 // Overly complex mod tree.....
pub struct Viewer;

impl Run for Viewer {
    fn initialize(&mut self, _application: &mut Application) -> Result<()> {
        info!("Viewer initialized");
        Ok(())
    }

    fn update(&mut self, _application: &mut Application) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let viewer = Viewer {};
    run_application(
        viewer,
        AppConfig {
            title: "Obsidian Viewer".to_string(),
            logfile_name: "viewer.log".to_string(),
            ..Default::default() //this syntax will fill in the rest of the properties with those specified in the Default method
        },
    )
}