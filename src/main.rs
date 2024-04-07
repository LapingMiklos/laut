use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::io;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut text: String = String::new();

    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Could not read from stdin");

        if line.starts_with("END") {
            break;
        }

        text += line.as_str();
    }

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
            '\u{30b}' => {
                if prev_char == ' ' {
                    laut_text += " ";
                }
            }
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
