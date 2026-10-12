fn main()
{
    // import input reader object + functionality
    use std::io;
    use std::io::Stdin;

    // import splitwhitespaceobject
    use std::str::SplitWhitespace;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // var to hold res
    let mut res: i32 = 0;

    // var to hold split input
    let mut nums: SplitWhitespace = "".split_whitespace();


    // read input and assin val to input var

    input_reader.read_line(&mut input).expect("Readline = Failed");


    //lesliecharleyhorse

    // split input 
    nums = input.split_whitespace();


    // loop to do geo math to find answer
    for loop_num in 0..2
        {
            // temp var to hold int value of the number
            let mut size: i32 = 1;

            // assign num val to size var
            size = nums.next().expect("Access Index = Failed").parse::<i32>().expect("Convert to i32 = Failed");


            // geo math
            res += size * 2

        }


    // result
    println!("{}", res);


}


        