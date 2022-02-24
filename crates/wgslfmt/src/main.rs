use std::io::Read;

fn main() -> Result<(), anyhow::Error> {
    let file = std::env::args().nth(1).filter(|arg| arg != "-");
    let input = match file {
        Some(file) => std::fs::read_to_string(&file)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let output = wgsl_formatter::format_str(&input);

    println!("{}", output);

    Ok(())
}
