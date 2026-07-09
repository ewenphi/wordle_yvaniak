use std::error::Error;

pub mod config;
mod gen_words;
mod ui;
use crate::ui::{cli::Cli, ChoixMenu, ResultPartie, Ui};
use config::{Config, ConfigUi};
use ui::UiEnum;

pub enum Instance {
    Cli,
    Tui,
}

pub struct App {
    instance: UiEnum,
}

impl App {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.instance.welcoming();
        loop {
            let partie_result = match self.instance.menu() {
                ChoixMenu::Quit => {
                    self.instance.quit();
                    return Ok(());
                }
                ChoixMenu::Start => self
                    .instance
                    .partie(gen_words::pick_the_word(), String::from("a")),
            };

            match partie_result {
                ResultPartie::Quit => {
                    self.instance.quit();
                    return Ok(());
                }
                ResultPartie::Stay => self.instance.welcoming(),
            }
        }
    }
    pub fn build(config: Config) -> Result<App, Box<dyn Error>> {
        match config.ui {
            ConfigUi::Cli => Ok(App {
                instance: UiEnum::ItemCli(Cli::new()),
            }),
            //ConfigUi::Tui => Ok(App {
            //    instance: UiEnum::ItemTui(Tui::new()),
            //}),
            _ => Err("tui and gui not implemented yet".into()),
        }
    }
}

pub fn launch(config: config::Config) -> Result<(), Box<dyn Error>> {
    let mut app = App::build(config)?;

    app.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: app run, fin build et launch
    #[test]
    fn app_build_with_config_cli_sucess() {
        let config = Config { ui: ConfigUi::Cli };
        let res = App::build(config);
        assert!(res.is_ok());
    }

    #[test]
    fn app_build_with_config_fail_because_gui_not_implemented() {
        let config = Config { ui: ConfigUi::Gui };
        let res = App::build(config);
        assert!(res.is_err());
    }

    #[test]
    fn launch_with_gui_error() {
        let config = Config { ui: ConfigUi::Gui };
        assert!(launch(config).is_err());
    }
}
