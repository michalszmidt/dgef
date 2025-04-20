use charset_normalizer_rs::from_path;
use std::path::PathBuf;

pub fn test_from_path(inp_pth: &str) {
    let result = from_path(&PathBuf::from(inp_pth), None).unwrap();
    let best_guess = result.get_best();
    let enc = best_guess.unwrap().encoding();

    println!("{enc}");
}
