mod game_window;

use std::{path::{Path, PathBuf}, error::Error};
use game_window::PCHardware;
use gbi::mainboard::Mainboard;

cfg_if::cfg_if!
{
    if #[cfg(feature = "gui")]
    {
        mod slint_uis;
        use slint_uis::*;

        use slint::*;
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        cfg_if::cfg_if!
        {
            if #[cfg(feature = "gui")]
            {
                let window = main_window::new();
                main_window::update_games(&window, &PathBuf::from("./")).unwrap();
                window.run();
                Ok(())
            }
            else
            {
                println!("Please provide a ROM file.");
                std::process::exit(1);
            }
        }
    }
    else
    {
        launch_game(String::from(&args[1]))
    }
}

pub fn launch_game(path: String) -> Result<(), Box<dyn Error>>
{
    let filename = PathBuf::from(path);
    let frontend = PCHardware::new()?;
    let mut motherboard = Mainboard::new(frontend);
    motherboard.load_game(Path::new(filename.as_path()))?;

    loop
    {
        motherboard.execute_frame();
    }
}
