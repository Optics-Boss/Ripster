use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};


fn main() -> io::Result<()>{
    let mut name_of_document = String::new();
    println!("Name of the document");
    io::stdin().read_line(&mut name_of_document)?;

    let file = File::create(&mut name_of_document.trim())
        .expect("!Error, the file name is incorrect");
    let mut writer = BufWriter::new(file);
    
    let mut document = String::new();
    println!("Ripster Text Editor");

    io::stdin().read_line(&mut document)?;
    let _ = writer.write_all(document.as_bytes());
    writer.flush()?;

    println!("{}", document);
    Ok(())
}

