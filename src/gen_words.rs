use rand::RngExt;

fn get_the_words() -> Vec<String> {
    ["thé", "café", "faculté", "lycée", "ordinateur", "téléphone"]
        .iter()
        .map(|m| String::from(*m))
        .collect()
}

pub fn pick_the_word() -> String {
    let dico: Vec<String> = get_the_words();
    //Teste que le dico n'est pas plus grand que usize
    assert!(size_of::<usize>() > dico.len());

    let mot: usize = rand::rng().random_range(0..=dico.len() - 1);

    assert!(mot < dico.len());
    dico[mot].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picked_word_is_in_dico() {
        let word: String = pick_the_word();
        let dico: Vec<String> = get_the_words();
        assert!(dico.contains(&word));
    }
}
