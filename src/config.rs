use clap::{builder::Str, crate_description, crate_name, crate_version, Arg, Command};

#[derive(Debug, PartialEq)]
pub enum ConfigUi {
    Cli,
    Tui,
    Gui,
}

fn new_subcmd(name: Str) -> Command {
    Command::new(name.clone())
        .about(format!("launch the wordle in the {} mode", name.clone()))
        .arg(Arg::new("quitting_test").hide(false).id("quitting_test"))
}

fn new_cmd() -> Command {
    Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(new_subcmd(Str::from("cli")))
        .subcommand(new_subcmd(Str::from("tui")))
        .subcommand(new_subcmd(Str::from("gui")))
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub ui: ConfigUi,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let lanceur = match args.next() {
            Some(arg) if arg == "cli" => ConfigUi::Cli,
            Some(arg) if arg == "tui" => ConfigUi::Tui,
            Some(arg) if arg == "gui" => ConfigUi::Gui,
            Some(_) => return Err("doesn't know this interface, the choices are gui, tui and cli"),
            None => ConfigUi::Cli,
        };

        Ok(Config { ui: lanceur })
    }

    pub fn cmd() -> Result<Config, &'static str> {
        let matches = new_cmd().get_matches();

        let lanceur = match matches.subcommand() {
            Some(("cli", _)) => ConfigUi::Cli,
            Some(("tui", _subm)) => ConfigUi::Tui,
            Some(("gui", _)) => ConfigUi::Gui,
            None => ConfigUi::Cli,
            Some((&_, _)) => {
                return Err("doesn't know this interface, the choices are gui, tui and cli")
            }
        };

        Ok(Config { ui: lanceur })
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn new_subcmd_correct_name() {
        let name = Str::from("cli");
        assert_eq!(new_subcmd(name).get_name(), "cli");
    }

    #[test]
    fn new_cmd_dont_panic() {
        assert_eq!(new_cmd().type_id(), TypeId::of::<Command>());
    }

    #[test]
    fn build_config_cli() {
        let args = ["", "cli"].iter().map(ToString::to_string);
        let config = Config::build(args);
        assert_eq!(config.unwrap(), Config { ui: ConfigUi::Cli });
    }

    #[test]
    fn build_config_tui() {
        let args = ["", "tui"].iter().map(|s| String::from(*s));
        let config = Config::build(args);
        assert_eq!(config.unwrap(), Config { ui: ConfigUi::Tui });
    }

    #[test]
    fn build_config_gui() {
        let args = ["", "gui"].iter().map(|s| String::from(*s));
        let config = Config::build(args);
        assert_eq!(config.unwrap(), Config { ui: ConfigUi::Gui });
    }

    #[test]
    fn build_config_vide() {
        let args = [""].iter().map(|s| String::from(*s));
        let config = Config::build(args);
        assert_eq!(config.unwrap(), Config { ui: ConfigUi::Cli });
    }

    #[test]
    fn build_config_unknown() {
        let args = ["", "unknown"].iter().map(|s| String::from(*s));
        let config = Config::build(args);
        assert!(config.is_err_and(|e| {
            e == "doesn't know this interface, the choices are gui, tui and cli"
        }),);
    }
}
