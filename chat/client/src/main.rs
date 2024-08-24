mod msg;
use std::io::stdin;
use std::io::Write;

use msg::MsgStore;

fn main() {
    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();

    
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let mut ms = MsgStore::new(buffer.trim().to_string());

    loop{
        ms.print(6);
        print!(">>> ");
        std::io::stdout().flush().unwrap();
   

        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        if buffer.trim() == "exit" {
            break;
        }

        ms.insert(buffer.trim().to_string());
        
    }
}
