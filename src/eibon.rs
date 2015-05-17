#![feature(convert)]            // .as_str

extern crate rustc_serialize;   // JSON
extern crate hyper;             // HTTP Client

use std::env;
use std::thread;
use std::collections::HashMap;

use std::io::Read;              // .read_to_string

use rustc_serialize::json::Json;
use hyper::Client;


fn search(word : String) -> String {
    // [Todo]
    // find word in cache
    // dictionaries API wrapper

    // let dict_url = "http://tw.dictionary.yahoo.com?p=";  // Yahoo
    let dict_url = "http://api.urbandictionary.com/v0/define?term=";    // Urban

    let query = format!("{}{}", dict_url, word.replace(" ", "+"));



    // send http request

    let mut client = Client::new();

    // Creating an outgoing request
    let mut res = client.get(query.as_str()).send().unwrap();

    // Read the Response
    let mut result = String::new();
    res.read_to_string(&mut result).unwrap();



    // parsing result (JSON)
    let result = Json::from_str(result.as_str()).unwrap();

    let result = format!("{}", result
                                .find("list").unwrap()[0]
                                .find("definition").unwrap());

    // remove redundant backslash for double quote
    let result = result.replace("\\\"", "\"");

    // [Todo]
    // remove first and last double quote
    // handle \r\n
    // return more info

    result
}

fn terminal_color(data : String) -> String {
    // [Todo]
    // make HashMap static, so no redundant calculation

    // VT100 colors

    let mut colors = HashMap::new();

    // reset
    colors.insert("reset",  "\x1b[0m");
    // foreground
    colors.insert("fg-black",   "\x1b[30m");
    colors.insert("fg-red",     "\x1b[31m");
    colors.insert("fg-green",   "\x1b[32m");
    colors.insert("fg-yellow",  "\x1b[33m");
    colors.insert("fg-blue",    "\x1b[34m");
    colors.insert("fg-magenta", "\x1b[35m");
    colors.insert("fg-cyan",    "\x1b[36m");
    colors.insert("fg-white",   "\x1b[37m");
    // background
    colors.insert("bg-black",   "\x1b[40m");
    colors.insert("bg-red",     "\x1b[41m");
    colors.insert("bg-green",   "\x1b[42m");
    colors.insert("bg-yellow",  "\x1b[44m");
    colors.insert("bg-blue",    "\x1b[45m");
    colors.insert("bg-magenta", "\x1b[46m");
    colors.insert("bg-cyan",    "\x1b[47m");
    colors.insert("bg-white",   "\x1b[48m");

    format!("{}{}{}", colors["fg-cyan"], data, colors["reset"])
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
