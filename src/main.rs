use chrono::{DateTime, Local};
use std::time::Duration;

// SOURCE OF THE ANSI CHAR - https://www.w3.org/TR/xml-entity-names/025.html
const  DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ "," ╻ "," ┏━┓ "," ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏━┓ "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ "," ┃ ","   ┃ ","   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ "," ┃ ","   ┃ ","   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ "," ┃ "," ┏━┛ "," ┣━┫ "," ┗━┫ "," ┗━┓ "," ┃━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ "," ┃ "," ┃   ","   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╻ "],
    ["┃ ┃ "," ┃ "," ┃   ","   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┗━┛ "," ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];



fn main() {

    loop{
        //clear screen
        print!("\x1b[2J");
        //Hides the cursor
        print!("\x1b[?25l");

        let t : DateTime<Local> = Local::now();
        //let t = t.format("%d/%m/%Y %H:%M:%S").to_string();
        let t = t.format("%H:%M:%S").to_string();
        // println!("Time is: {:?}", t);
        println!("Time is:");

        for row in &DIGITS{
            for c in t.chars(){
                let col:usize = match c{
                    '0'..='9' => c as usize - '0' as usize,
                    ';' => 10,
                    _ => 10,
                };
                print!("{}", row[col]);
            }
            println!();
        }

        std::thread::sleep(Duration::from_millis(999));

        //Moves cursor up 7times to ensure we do not print newlines
        print!("\x1b[7A");
    }
}

//cargo install --path .
//To install to your computer