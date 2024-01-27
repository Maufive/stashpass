use std::io::{ Write, BufRead, stdout };

pub fn print<W: Write>(writer: &mut W, message: &str) {
    writeln!(writer, "{message}").unwrap_or_else(|_| println!("{message}"));
}

pub fn read_terminal_input<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    prompt: Option<&str>
) -> String {
    if let Some(prompt) = prompt {
        write!(writer, "{}", prompt).unwrap();
    }

    write!(writer, "{}", "").unwrap_or_else(|_| print!("{}", "👉 "));
    stdout().flush().unwrap();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_owned()
}
