fn main()
{
    // import input read and input reader object
    use std::io;
    use std::io::Stdin;
    use std::str::Chars;
    //lesliecharleyhorse


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var for input
    let mut input: String = String::new();

  

    // readline
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // iterator for digits 
    let mut digits:Chars<> = input.chars();

    for dig in digits
        {
            if dig == '-'
                {
                    continue;
                }
            
            println!("{}", dig);
        }
}