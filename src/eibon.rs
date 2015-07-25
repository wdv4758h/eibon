#![feature(convert)]            // .as_str

extern crate rustc_serialize;   // JSON
extern crate hyper;             // HTTP Client

use std::env;
use std::thread;
use std::io::Read;              // .read_to_string

use rustc_serialize::json::Json;
use hyper::Client;

trait VT100 {
    fn vt100(&self, color: &'static str) -> String;
}

impl VT100 for String {
    fn vt100(&self, color: &'static str) -> String {
        let code = match color {
            "bold"          => 1,
            // foreground
            "fg-black"      => 30,
            "fg-red"        => 31,
            "fg-green"      => 32,
            "fg-yellow"     => 33,
            "fg-blue"       => 34,
            "fg-magenta"    => 35,
            "fg-cyan"       => 36,
            "fg-white"      => 37,
            // background
            "bg-black"      => 40,
            "bg-red"        => 41,
            "bg-green"      => 42,
            "bg-yellow"     => 43,
            "bg-blue"       => 44,
            "bg-magenta"    => 45,
            "bg-cyan"       => 46,
            "bg-white"      => 47,
            // reset
            "reset"         => 0,
            _               => 0,
        };

        format!("\x1b[{}m{}\x1b[0m", code, self.trim_right_matches("\x1b[0m"))
    }
}

trait Dictionary {
    fn send_query(word: &String) -> String;
    fn parse(word: &String, data: &String) -> String;
    fn search(word: &String) -> String;
}

struct YahooDict;
struct UrbanDict;

impl Dictionary for UrbanDict {
    fn send_query(word: &String) -> String {
        let api = "http://api.urbandictionary.com/v0/define?term=";
        let query = format!("{}{}", api, word.replace(" ", "+"));

        // send http request
        let mut client = Client::new();

        // Creating an outgoing request
        let mut res = client.get(query.as_str()).send().unwrap();

        // Read the Response
        let mut result = String::new();
        res.read_to_string(&mut result).unwrap();

        result
    }

    fn parse(word: &String, data: &String) -> String {

        // parsing result (JSON)
        let result = Json::from_str(data.as_str()).unwrap();

        let ref result0 = result.find("list").unwrap()[0];

        let definition = format!("{}", result0.find("definition").unwrap());
        let example = format!("{}", result0.find("example").unwrap());

        // remove redundant backslash for double quote
        let definition = definition
                            .replace("\\\"", "\"")
                            .replace("\\r", "\r")
                            .replace("\\n", "\n");

        let example = example
                        .replace("\\\"", "\"")
                        .replace("\\r", "\r")
                        .replace("\\n", "\n");

        let result = format!("\n{}\n{}\n{}\n",
                                word.vt100("bold").vt100("fg-yellow"),
                                definition.vt100("fg-cyan"),
                                example);

        result
    }

    fn search(word: &String) -> String {
        let result = UrbanDict::send_query(&word);
        let result = UrbanDict::parse(&word, &result);
        result
    }
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
                move || { UrbanDict::search(&word) }
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
