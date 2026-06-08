use mini_grep::{cli_analyse, file_reader, find_matches};

fn main() {
    let (mots, chemin) = cli_analyse();
    let content = match file_reader(chemin) {
        Ok(value) => value,
        Err(e) => {
            println!("erreur de lecture du fichiers: {}", e);
            return;
        }
    };
    let result = find_matches(content.as_str(), &mots);
    for test in result {
        println!("{}", test);
    }
}
