use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let mut x = 4;
    println!("x is: {}", x);

    {
        // Use the x value to do ting
        let x = x - 2;
        println!("x is: {}", x);
        // does not chance the x vlaue outside of {}
    }

    // use the top x value not the one that is insite {}
    x = x + 1;
    println!("x is: {}", x);

    // can make the x value to a string value
    let x = "Hello";
    println!("x is: {}", x);

    const SECONDS_IN_MINUTE: i32 = 60;
    // const SECONDS_IN_MINUTE: i32 = 10; // can't chance the value of SECONDS_IN_MINUTE when u use const
    println!("{}", SECONDS_IN_MINUTE);

    //integers is number for how many bits we are going to use.
    // integers we have
    // i8
    // i16
    // i32
    // i64
    // i128

    let i: u32 = 900;
    println!("i is: {}", i);
    // u32 is a unsigned integer
    // unsigned integers we have
    // u8
    // u16
    // u32
    // u64
    // u128

    //floting point value
    // f32
    // f64
    //exempel:
    let floting_point: f32 = 10.9;
    println!("floting_point is: {}", floting_point);

    //  boolean
    //  boolean can only be true ore false value
    let true_or_false: bool = false;
    println!("true_or_false is: {}", true_or_false);

    //char
    // char can only store ting ting
    let letter: char = 'a';
    println!("letter is: {}", letter);

    read_dir("../bassomc/src/html");

    //made a webserver with rust that will render a html file and it wil run on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn read_dir(path: &str) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let exclude_files = ["target", ".git"];
        if path.is_dir() {
            if exclude_files.contains(&path.file_name().unwrap().to_str().unwrap()) {
                continue;
            }
            println!("Directory: {}", path.display());
            read_dir(&path.display().to_string())
        } else {
            println!("File: {}", path.display());
        }
    }
}




// This code is making a discord tamplate file
// use std::io::stdin;
// use std::fs::File;
// use std::io::prelude::*;

// fn create_js_bot_template() -> std::io::Result<()> {
//     let mut file = File::create("js_bot_template.js")?;
//     file.write_all(b"const Discord = require('discord.js');\n\nconst client = new Discord.Client();\n\nclient.on('ready', () => {\n  console.log(`Logged in as ${client.user.tag}!`);\n});\n\nclient.on('message', msg => {\n  if (msg.content === 'ping') {\n    msg.reply('Pong!');\n  }\n});\n\nclient.login('your-token-goes-here');")?;
//     println!("JavaScript-based bot template created successfully!");
//     Ok(())
// }


// fn create_ts_bot_template() -> std::io::Result<()> {
//     let mut file = File::create("ts_bot_template.ts")?;
//     file.write_all(b"import { Client } from 'discord.js';\n\nconst client = new Client();\n\nclient.on('ready', () => {\n  console.log(`Logged in as ${client.user.tag}!`);\n});\n\nclient.on('messageCreate', msg => {\n  if (msg.content === 'ping') {\n    msg.reply('Pong!');\n  }\n});\n\nclient.login('your-token-goes-here');")?;
//     println!("TypeScript-based bot template created successfully!");
//     Ok(())
// }


// fn main() {
//     loop {
//         println!("Please select a bot template type:\n1. JavaScript-based\n2. TypeScript-based");

//         let mut input = String::new();

//         stdin()
//             .read_line(&mut input)
//             .expect("Failed to read user input");

//         let template_type: u32 = match input.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         match template_type {
//             1 => {
//                 if let Err(e) = create_js_bot_template() {
//                     eprintln!("Error creating JS bot template: {:?}", e);
//                 } else {
//                     break;
//                 }
//             },
//             2 => {
//                 if let Err(e) = create_ts_bot_template() {
//                     eprintln!("Error creating TS bot template: {:?}", e);
//                 } else {
//                     break;
//                 }
//             },
//             _ => break,
//         }
//     }
// }





