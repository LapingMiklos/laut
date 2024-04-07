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
    text += "??";

    let mut laut_text: String = String::new();

    let char_vec = text.chars().collect::<Vec<char>>();
    let mut windows = char_vec.windows(3);

    let to_laut = |c: &char| match c {
        'o' => 'ő',
        'O' => 'Ő',
        'u' => 'ű',
        'U' => 'Ű',
        _ => c.to_owned(),
    };

    while let Some(chars) = windows.next() {
        match chars {
            [' ', '\u{30b}', t] => {
                laut_text += &to_laut(t).to_string();
                windows.next();
                windows.next();
            }
            [f, _, _] => laut_text += &format!("{}", f),
            _ => {}
        }
    }

    println!("{}", laut_text);

    match ctx.set_contents(laut_text) {
        Ok(_) => println!("Copied to clipboard!"),
        Err(_) => println!("Could not copy to clipboard"),
    }
}
