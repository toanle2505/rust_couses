use std::{fs::{self, File, OpenOptions},io::{self, BufRead, BufWriter, Read, Write}};
fn main() -> io::Result<()> {
    let mut file = match File::open("file.txt")  {
        Ok(f) => f  ,
        Err(e) => {
            print!("Error opening file: {}", e);
            return  Err(e);
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("File content: {}", content);

    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .append(true)
    .open("new_file.txt")?;

    file.write(content.as_bytes())?;

    Ok(())

}

fn process_large_file() -> io::Result<()> {
    let file = File::open("large_file.txt")?;
    let mut writter = BufWriter::new(file);

    for i in 0..1000 {
        writter.write_all(format!("Line {}\n", i).as_bytes())?;
    }
    writter.flush()?;

    let file = File::open("large_file.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? > 0 {
        println!("{}", buffer);
        buffer.clear();
    }

    Ok(())
}

fn process_directory() -> io::Result<()> {
    let entries = fs::read_dir(".")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("File: {}", path.display());
        } else if path.is_dir() {
            println!("Directory: {}", path.display());
        }
    }
    Ok(())
}
