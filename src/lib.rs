// use std::{io, string};
// use  std :: io :: BufReader ;
// use  std :: fs :: File ;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

pub fn cli_analyse() -> (String, std::path::PathBuf) {
    let args = Cli::parse();
    let (pattern, path) = (args.pattern, args.path);
    (pattern, path)
}

pub fn file_reader(path: std::path::PathBuf,) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

pub fn find_matches(content: &str, pattern: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .map(|line| line.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let content = "rust est rapide\njava est lent\nrust est sûr";
        let pattern = "rust";

        let result = find_matches(content, pattern);

        assert_eq!(
            result,
            vec![
                "rust est rapide".to_string(),
                "rust est sûr".to_string()
            ]
        );
    }

    #[test]
    fn test_no_match() {
        let content = "java\npython";
        let pattern = "rust";

        let result = find_matches(content, pattern);

        assert!(result.is_empty());
    }

    #[test]
    fn test_empty_content() {
        let result = find_matches("", "rust");

        assert!(result.is_empty());
    }

    #[test]
    fn test_multiple_matches() {
        let content = "rust\nrustacean\nrust";
        let pattern = "rust";

        let result = find_matches(content, pattern);

        assert_eq!(
            result,
            vec![
                "rust".to_string(),
                "rustacean".to_string(),
                "rust".to_string()
            ]
        );
    }

    #[test]
    fn test_case_sensitive() {
        let content = "Rust\nrust\nRUST";

        let result = find_matches(content, "rust");

        assert_eq!(
            result,
            vec![
                "rust".to_string()
            ]
        );
    }
}

// les teste pour reafile 
#[test]
fn test_file_reader() {
    let path = std::path::PathBuf::from("test.txt");

    std::fs::write(&path, "bonjour").unwrap();

    let content = file_reader(path.clone()).unwrap();

    assert_eq!(content, "bonjour");

    std::fs::remove_file(path).unwrap();
}

// teste pour les cas d'erreur
#[test]
fn test_file_reader_error() {
    let path = std::path::PathBuf::from("fichier_inexistant.txt");

    let result = file_reader(path);

    assert!(result.is_err());
}