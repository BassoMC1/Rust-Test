fn main() {
        let mut x = 4;
        println!("x is: {}", x);

        {
            // Use the x value to do ting
            let x  =  x  - 2;
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
    // const SECONDS_IN_MINUTE: i32 = 60; // does not work when u use const
    print!("{}", SECONDS_IN_MINUTE)
}
