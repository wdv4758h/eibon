use std::env;

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
    for word in words {
        let result = search(word);
        let result = terminal_color(result);
        response(result);
    }
}
