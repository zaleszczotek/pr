use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(line_content) => {

                if let Some((address_part, value_part)) = line_content.split_once(' ') {
                    if address_part.starts_with('[') /*&& address_part.ends_with(']')*/ {
                        let address = &address_part[1..address_part.len()-1]; // stripping '[' and ']'
                        let cleaned_address: String = address.chars()
                            .filter(|&c| c != ']')
                            .collect();
                        let value = value_part;
                        println!("{}, {}", cleaned_address, value);
                    } else {
                        // log
                        //eprintln!("Invalid format: {}", line_content);
                    }
                } else {
                    // log
                    //eprintln!("Invalid format: {}", line_content);
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

}
