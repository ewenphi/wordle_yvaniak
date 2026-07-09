use cli::Cli;
use std::error::Error;
use std::str::Chars;
//use tui::Tui;
use unicode_segmentation::UnicodeSegmentation;
pub mod cli;
//pub mod tui;

pub enum ChoixMenu {
    Start,
    Quit,
}

pub enum UiEnum {
    ItemCli(Cli),
    // ItemTui(Tui),
}

impl UiEnum {
    pub fn welcoming(&self) {
        match self {
            //UiEnum::ItemTui(tui) => tui.welcoming(),
            UiEnum::ItemCli(cli) => cli.welcoming(),
        }
    }

    pub fn menu(&mut self) -> ChoixMenu {
        match self {
            //UiEnum::ItemTui(tui) => tui.menu(),
            UiEnum::ItemCli(cli) => cli.menu(),
        }
    }

    pub fn partie(&mut self, mot: String, guess_test: String) -> ResultPartie {
        match self {
            //UiEnum::ItemTui(tui) => tui.partie(mot, guess_test),
            UiEnum::ItemCli(cli) => cli.partie(mot, guess_test),
        }
    }

    pub fn quit(&mut self) {
        match self {
            //UiEnum::ItemTui(tui) => tui.quit(),
            UiEnum::ItemCli(cli) => cli.quit(),
        }
    }
}

pub trait Ui {
    fn new() -> Self;

    fn welcoming(&self);

    fn menu(&mut self) -> ChoixMenu;

    fn partie(&mut self, mot: String, guess_test: String) -> ResultPartie;

    fn quit(&mut self);
}

#[derive(Debug, PartialEq)]
pub enum ResultPlacement {
    Good(char),
    Misplaced(char),
    Bad(char),
}

#[derive(Debug, PartialEq)]
pub enum ResultWordle {
    Win,
    UnmatchedLens(usize, usize),
    Placement(Placement),
}

#[derive(Debug, PartialEq)]
pub enum ResultPartie {
    Quit,
    Stay,
}

//TODO:Verifier que ça peut vraiment renvoyer une erreur
pub fn traitement_wordle(ref_mot: &String, guess: String) -> Result<ResultWordle, Box<dyn Error>> {
    if guess == *ref_mot {
        return Ok(ResultWordle::Win);
    }

    let len_guess = guess.graphemes(true).count();
    let len_mot = (*ref_mot).graphemes(true).count();
    if len_guess != len_mot {
        return Ok(ResultWordle::UnmatchedLens(len_mot, len_guess));
    }

    assert!(len_guess == len_mot);

    Placement::build(ref_mot, guess)
}

#[derive(Debug, PartialEq)]
pub struct Placement {
    result: Vec<ResultPlacement>,
}

impl Placement {
    fn build(ref_mot: &String, guess: String) -> Result<ResultWordle, Box<dyn Error>> {
        let mut result: Vec<ResultPlacement> = Vec::new();
        let chars_mot = (*ref_mot).chars();
        let chars_guess = guess.chars();

        for (i_l, l) in chars_guess.clone().enumerate() {
            if !(*ref_mot).contains(l) {
                result.push(ResultPlacement::Bad(l));
            } else if chars_mot.clone().nth(i_l).unwrap() == l {
                result.push(ResultPlacement::Good(l));
            } else {
                let letters_well_placed: u32 =
                    count_of_this_letter_well_placed(l, &chars_mot, &chars_guess);

                let letters_not_well_placed: u32 =
                    count_of_this_letter_total(l, &chars_mot, &chars_guess);

                if letters_not_well_placed - letters_well_placed < 2 {
                    result.push(ResultPlacement::Bad(l));
                } else {
                    result.push(ResultPlacement::Misplaced(l));
                }
            }
        }

        Ok(ResultWordle::Placement(Placement { result }))
    }
}

fn count_of_this_letter_well_placed(
    l: char,
    chars_mot: &Chars<'_>,
    chars_guess: &Chars<'_>,
) -> u32 {
    let mut cpt: u32 = 0;
    for (i_mot, l_mot) in chars_mot.clone().enumerate() {
        if l == l_mot && chars_guess.clone().nth(i_mot).unwrap() == l {
            cpt += 1;
        }
    }
    cpt
}

fn count_of_this_letter_total(l: char, chars_mot: &Chars<'_>, chars_guess: &Chars<'_>) -> u32 {
    let mut cpt: u32 = 0;
    for (i_mot, l_mot) in chars_mot.clone().enumerate() {
        if l == l_mot || chars_guess.clone().nth(i_mot).unwrap() == l {
            cpt += 1;
        }
    }
    cpt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placement_brass_sands() {
        let res = Placement::build(&String::from("brass"), String::from("sands"));
        match res.unwrap() {
            ResultWordle::Placement(p) => {
                assert_eq!(
                    p.result,
                    vec![
                        ResultPlacement::Misplaced('s'),
                        ResultPlacement::Misplaced('a'),
                        ResultPlacement::Bad('n'),
                        ResultPlacement::Bad('d'),
                        ResultPlacement::Good('s'),
                    ]
                )
            }
            _ => panic!("shouldn't have other than Placement"),
        }
    }

    #[test]
    fn placement_brass_turns() {
        let res = Placement::build(&String::from("brass"), String::from("turns"));
        match res.unwrap() {
            ResultWordle::Placement(p) => {
                assert_eq!(
                    p.result,
                    vec![
                        ResultPlacement::Bad('t'),
                        ResultPlacement::Bad('u'),
                        ResultPlacement::Misplaced('r'),
                        ResultPlacement::Bad('n'),
                        ResultPlacement::Good('s'),
                    ]
                )
            }
            _ => panic!("shouldn't have other than Placement"),
        }
    }

    #[test]
    fn placement_brass_super() {
        let res = Placement::build(&String::from("brass"), String::from("super"));
        match res.unwrap() {
            ResultWordle::Placement(p) => {
                assert_eq!(
                    p.result,
                    vec![
                        ResultPlacement::Misplaced('s'),
                        ResultPlacement::Bad('u'),
                        ResultPlacement::Bad('p'),
                        ResultPlacement::Bad('e'),
                        ResultPlacement::Misplaced('r'),
                    ]
                )
            }
            _ => panic!("shouldn't have other than Placement"),
        }
    }

    #[test]
    fn placement_brass_carbs() {
        let res = Placement::build(&String::from("brass"), String::from("carbs"));
        match res.unwrap() {
            ResultWordle::Placement(p) => {
                assert_eq!(
                    p.result,
                    vec![
                        ResultPlacement::Bad('c'),
                        ResultPlacement::Misplaced('a'),
                        ResultPlacement::Misplaced('r'),
                        ResultPlacement::Misplaced('b'),
                        ResultPlacement::Good('s'),
                    ]
                )
            }
            _ => panic!("shouldn't have other than Placement"),
        }
    }

    #[test]
    fn placement_brass_barbs() {
        let res = Placement::build(&String::from("brass"), String::from("barbs"));
        match res.unwrap() {
            ResultWordle::Placement(p) => {
                assert_eq!(
                    p.result,
                    vec![
                        ResultPlacement::Good('b'),
                        ResultPlacement::Misplaced('a'),
                        ResultPlacement::Misplaced('r'),
                        ResultPlacement::Bad('b'),
                        ResultPlacement::Good('s'),
                    ]
                )
            }
            _ => panic!("shouldn't have other than Placement"),
        }
    }

    #[test]
    fn placement_brass_canal() {
        let res = Placement::build(&String::from("brass"), String::from("canal"));
        match res.unwrap() {
            ResultWordle::Placement(p) => {
                assert_eq!(
                    p.result,
                    vec![
                        ResultPlacement::Bad('c'),
                        ResultPlacement::Misplaced('a'),
                        ResultPlacement::Bad('n'),
                        ResultPlacement::Misplaced('a'),
                        ResultPlacement::Bad('l'),
                    ]
                )
            }
            _ => panic!("shouldn't have other than Placement"),
        }
    }

    #[test]
    fn placement_telephone_teephone() {
        let res = Placement::build(&String::from("téléphone"), String::from("téééphone"));
        match res.unwrap() {
            ResultWordle::Placement(p) => {
                assert_eq!(
                    p.result,
                    vec![
                        ResultPlacement::Good('t'),
                        ResultPlacement::Good('é'),
                        ResultPlacement::Bad('é'),
                        ResultPlacement::Good('é'),
                        ResultPlacement::Good('p'),
                        ResultPlacement::Good('h'),
                        ResultPlacement::Good('o'),
                        ResultPlacement::Good('n'),
                        ResultPlacement::Good('e'),
                    ]
                )
            }
            _ => panic!("shouldn't have other than Placement"),
        }
    }
}
