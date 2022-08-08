use rand::prelude::*;
use std::io;
use std::io::stdin;
use std::io::Write;
#[warn(unused_must_use)]
use std::mem::replace;

fn main() {
    let first = r#" 
     ____
    |/   |
    |   
    |    
    |    
    |    
    |
    |_____
    "#;
    let second = r#" 
    ____
   |/   |
   |   (_)
   |    
   |    
   |    
   |
   |_____
   "#;
    let third = r#"
     ____
    |/   |
    |   (_)
    |    |
    |    |    
    |    
    |
    |_____
    "#;
    let fourth = r#"
     ____
    |/   |
    |   (_)
    |   \|
    |    |
    |    
    |
    |_____
    "#;
    let five = r#" 
     ____
    |/   |
    |   (_)
    |   \|/
    |    |
    |    
    |
    |_____
    "#;
    let six = r#" 
     ____
    |/   |
    |   (_)
    |   \|/
    |    |
    |   / 
    |
    |_____
    "#;
    let seven = r#"
     ____
    |/   |
    |   (_)
    |   \|/
    |    |
    |   / \
    |
    |_____
    "#;

    let words = vec!["batman", "action", "active", "actual"];
    let random_word = words.choose(&mut thread_rng()).unwrap().to_string();
    // println!("{}", random_word);

    let mut found_letters = vec![
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
    ];

    let hangman_title = r#"
     _____  _    _  _____ _______   _    _          _   _  _____ __  __          _   _  
    |  __ \| |  | |/ ____|__   __| | |  | |   /\   | \ | |/ ____|  \/  |   /\   | \ | | 
    | |__) | |  | | (___    | |    | |__| |  /  \  |  \| | |  __| \  / |  /  \  |  \| | 
    |  _  /| |  | |\___ \   | |    |  __  | / /\ \ | . ` | | |_ | |\/| | / /\ \ | . ` | 
    | | \ \| |__| |____) |  | |    | |  | |/ ____ \| |\  | |__| | |  | |/ ____ \| |\  | 
    |_|  \_\\____/|_____/   |_|    |_|  |_/_/    \_\_| \_|\_____|_|  |_/_/    \_\_| \_|                                                                                
"#;

    println!("{}", hangman_title);
    // println!("{}", random_word);

    let mut trys = 6;

    while trys > 0 {
        if trys == 6 {
            println!("{}", seven);
        } else if trys == 5 {
            println!("{}", six);
        } else if trys == 4 {
            println!("{}", five);
        } else if trys == 3 {
            println!("{}", fourth);
        } else if trys == 2 {
            println!("{}", third);
        } else if trys == 1 {
            println!("{}", second);
        } else if trys == 0 {
            println!("{}", first);
        }
        println!(
            "    {} {} {} {} {} {}",
            found_letters[0],
            found_letters[1],
            found_letters[2],
            found_letters[3],
            found_letters[4],
            found_letters[5]
        );

        let mut input_string = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        stdin()
            .read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");

        if random_word.contains(input_string.trim()) {
            for n in 0..random_word.len() {
                if random_word.chars().nth(n).unwrap().to_string()
                    == input_string.trim().to_string()
                {
                    let _ = replace(&mut found_letters[n], input_string.trim().to_string());
                }
            }
        } else {
            trys -= 1;
        }
    }
}
