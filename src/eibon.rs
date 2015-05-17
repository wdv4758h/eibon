use std::env;
use std::thread;

fn search(word : String) -> String {
    // [Todo]
    // find word in cache
    // send request to Yahoo dictionary
    format!("{} search", word)
}

fn terminal_color(word : String) -> String {
    // [Todo]
    // static variable (dict) for VT100 colors
    format!("{} color", word)
}

fn response(data : String) {
    println!("{}", data);
}

fn main() {
    let args = env::args().skip(1);
    let mut words : Vec<String> = vec![];

    // parsing arguments
    for arg in args {
        match arg {
            // [Todo]
            // more arguments support
            word => words.push(word),
        }
    }

    // searching
    // if there are mutiple words, use thread and then join the result

    let mut children : Vec<_> = vec![];

    for word in words {
        children.push(
            thread::spawn(
                move || { terminal_color(search(word)) }
            )
        )
    }

    for child in children {
        let result = child.join();
        match result {
            Ok(data) => response(data),
            Err(_) => {},
        }
    }
}
