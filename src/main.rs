use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("File not found.");
            return;
        }
    };

    let mut buffer = String::new();
    let mut cursor_pos = buffer.len();

    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            buffer.push_str(&line);
            buffer.push('\n');
        }
    }

    loop {
        print_buffer(&buffer, &cursor_pos);

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Error reading input.");
                return;
            }
        }

        let input = input.trim();
        match input {
            ":q" => break,
            ":w" => {
                save_buffer_to_file(&filename, &buffer);
                println!("File saved.");
            }
            ":wq" => {
                save_buffer_to_file(&filename, &buffer);
                println!("File saved.");
                break;
            }
            ":a" => {
                cursor_pos = buffer.len();
                println!("Insert mode activated.");
            }
            _ => {
                if input.starts_with(':') {
                    println!("Invalid command.");
                } else {
                    buffer.insert_str(cursor_pos, input);
                    cursor_pos += input.len();
                    buffer.insert(cursor_pos, '\n');
                    cursor_pos += 1;
                }
            }
        }
    }
}

fn print_buffer(buffer: &str, cursor_pos: &usize) {
    print!("\x1B[2J\x1B[1;1H");
    println!("Type :q to quit, :w to save, :wq to save and quit, :a to insert text at the end of the file.\n");
    print!("{}", buffer);
    io::stdout().flush().unwrap();
}

fn save_buffer_to_file(filename: &str, buffer: &str) {
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
    {
        Ok(file) => file,
        Err(_) => {
            println!("Error creating file.");
            return;
        }
    };
    match file.write_all(buffer.as_bytes()) {
        Ok(_) => (),
        Err(_) => println!("Error writing to file."),
    }
}