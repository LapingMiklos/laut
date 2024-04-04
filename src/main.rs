use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::env;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let text: String = env::args()
        .skip(1)
        .next()
        .expect("Expected cmdline argument");

    let mut prev_char: char = '?';
    let mut laut_text: String = String::new();

    let handle_laut = |prev: char, curr: char| match prev {
        ' ' => format!(" {}", curr),
        '\u{30b}' => match curr {
            'o' => String::from("ő"),
            'u' => String::from("ű"),
            'O' => String::from("Ő"),
            'U' => String::from("Ű"),
            _ => format!("{}", curr),
        },
        _ => format!("{}", curr),
    };

    for char in text.chars() {
        match char {
            ' ' => {}
            '\u{30b}' => {}
            _ => {
                laut_text += handle_laut(prev_char, char).as_str();
            }
        }
        prev_char = char;
    }

    println!("{}", laut_text);

    match ctx.set_contents(laut_text) {
        Ok(_) => println!("Copied to clipboard!"),
        Err(_) => println!("Could not copy to clipboard"),
    }
}
