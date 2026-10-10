fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();


    // var to hold input
    let mut input: String = String::new();

    // var to hold int value of input
    let mut input_int: i32 = 0;


    // readline
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // assign int value 
    input_int = input.trim().parse::<i32>().expect("Convert to int = Failed");


    // conditionals to fin answer
    if input_int <= 3
        {println!("Initial");}
    else if input_int <= 6
        {println!("Average");}
    else if input_int <= 9
        {println!("Sufficient");}
    else if input_int <= 12
        {println!("High");}
}