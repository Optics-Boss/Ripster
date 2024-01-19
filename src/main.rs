use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::io::ErrorKind;
use std::env;


fn main() -> io::Result<()>{
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 1 {
        panic!("Incorrect amount of arguments has been givin");
    }

    let name_of_document = String::from(&arguments[1]);

    let file = File::open(&mut name_of_document.trim()).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&mut name_of_document.trim())
                .expect("!Error, the file name is incorrect")
        } else {
            panic!("Something went wrong");
        }
    });


    let mut writer = BufWriter::new(file);
    
    let mut document = String::new();
    println!("Ripster Text Editor");

    io::stdin().read_line(&mut document)?;
    let _ = writer.write_all(document.as_bytes());
    writer.flush()?;

    println!("{}", document);
    Ok(())
}

