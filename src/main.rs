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

    //made a webserver with rust that will render a html file and it wil run on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }



}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let file_path = "src/hello.html";
    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}