use super::{traitement_wordle, ResultPartie, ResultPlacement, ResultWordle};
use super::{ChoixMenu, Ui};
use colored::Colorize;
pub struct Cli {}

impl Ui for Cli {
    fn new() -> Self {
        Self {}
    }

    fn quit(&mut self) {}

    fn welcoming(&self) {
        match cliclack::intro("Welcome in the menu of this wordle game !") {
            Ok(_) => {}
            Err(e) => eprintln!("An error happened during the print of the intro message : {e}"),
        }
    }

    fn menu(&mut self) -> ChoixMenu {
        let _choix: String = String::new();

        loop {
            let ans = cliclack::select("What do you want to do ?")
                .initial_value("Start a game")
                .item("start", "Start a game", "")
                .item("quit", "Quit the game", "")
                .interact();

            match ans {
                Ok(choice) => match choice {
                    "start" => return ChoixMenu::Start,
                    "quit" => {
                        match cliclack::outro("Exiting") {
                            Ok(_) => {}
                            Err(e) => eprintln!(
                                "An error happened during the print of the outro message : {e}"
                            ),
                        }
                        return ChoixMenu::Quit;
                    }
                    _ => eprintln!("error happened during the choice in the menu"),
                },
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::Interrupted {
                        eprintln!("There was an error, please try again")
                    }
                }
            }
        }
    }

    fn partie(&mut self, mot: String, _guess: String) -> ResultPartie {
        let _ = cliclack::log::info(format!(
            "The wordle game begin ! The word has {} letters",
            mot.chars().count()
        ));

        match cliclack::log::info(
            "You can go to the menu by inputting : m and exit by inputting : e",
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("An error happened during the print of the info of menu and quit : {e}")
            }
        }

        loop {
            let taille = mot.chars().count();
            let mot_cloned = mot.clone();

            let res = cliclack::input(format!(
                "What is your guess for the word of {taille} letters"
            ))
            .placeholder(&"*".repeat(taille))
            .validate_interactively(move |input: &String| {
                if input.is_empty() {
                    Err("Please enter an answer.")
                } else if input.chars().count() < taille && input != "e" && input != "m" {
                    Err("Too short")
                } else if input.chars().count() > taille && input != "e" && input != "m" {
                    Err("Too long")
                } else {
                    Ok(())
                }
            })
            .validate(move |input: &String| {
                match traitement_wordle(&mot_cloned, input.to_string()) {
                    Ok(ResultWordle::Placement(placement)) => {
                        let mut message = String::new();
                        for i in placement.result.into_iter() {
                            match i {
                                ResultPlacement::Misplaced(l) => {
                                    message.push_str(
                                        format!(
                                            "{}",
                                            l.to_string().as_str().truecolor(255, 165, 0)
                                        )
                                        .as_str(),
                                    );
                                }
                                ResultPlacement::Bad(l) => {
                                    message.push_str(
                                        format!("{}", Colorize::red(l.to_string().as_str()))
                                            .as_str(),
                                    );
                                }
                                ResultPlacement::Good(l) => {
                                    message.push_str(
                                        format!("{}", Colorize::green(l.to_string().as_str()))
                                            .as_str(),
                                    );
                                }
                            }
                        }
                        Err(message)
                    }
                    Ok(ResultWordle::Win) => Ok(()),
                    Ok(ResultWordle::UnmatchedLens(_len_mot, len_guess)) => {
                        if len_guess == 1 {
                            return Ok(());
                        }
                        Err(String::from(""))
                    }
                    Err(_) => Err(String::from("")),
                }
            })
            .interact();

            let guess = match res {
                Ok(guess) => guess,
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::Interrupted => {}
                        _ => eprint!("error {} happened, try again : {}", e, e.kind(),),
                    }
                    String::from("")
                }
            };

            if guess.is_empty() {
                continue;
            }

            let guess = guess.trim();

            if guess == "e" {
                match cliclack::outro("Exiting") {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("An error happened during the print of the outro message : {e}")
                    }
                }
                return ResultPartie::Quit;
            }

            if guess == "m" {
                match cliclack::log::info("\nGoing to menu") {
                    Ok(_) => {}
                    Err(e) => eprintln!(
                        "An error happened during the print of the going to menu message : {e}"
                    ),
                }
                return ResultPartie::Stay;
            }

            let guess = String::from(guess);

            match traitement_wordle(&mot, guess) {
                Ok(ResultWordle::Win) => {
                    match cliclack::log::success("You win !") {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error happenned during the print of the win message : {e}")
                        }
                    }
                    return ResultPartie::Stay;
                }
                Ok(ResultWordle::UnmatchedLens(len_mot, len_guess)) => {
                    println!("You guessed a word of {len_guess} letters but the word to guess contains {len_mot} letters.");
                }
                Ok(ResultWordle::Placement(placement)) => {
                    for (cpt, i) in placement.result.into_iter().enumerate() {
                        match i {
                            ResultPlacement::Misplaced(l) => {
                                println!("\nThe letter {l} in position {cpt} is misplaced");
                            }
                            ResultPlacement::Bad(l) => {
                                println!("\nThe letter {l} in position {cpt} is not good");
                            }
                            ResultPlacement::Good(_l) => {}
                        }
                    }
                }
                Err(_e) => continue,
            }
        }
    }
}
