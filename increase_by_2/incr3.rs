fn main()
{
    // import input read functionality + input reader object
    use std::io;
    use std::io::Stdin;

    // import splitwhitespace iterator
    use std::str::SplitWhitespace;
    //lesliecharleyhorse



    

    // input reader object
    //lesliecharleyhorse
    let input_reader: Stdin = io::stdin();

    // var to hold input 
    let mut input: String = String::new();


    // iterator to hold nums 
    let mut nums: SplitWhitespace = "".split_whitespace();

    // result string 
    let mut res: String = String::new();

    


    // readline skip this
    input_reader.read_line(&mut input).expect("Readline = Failed");
    input.clear();


    // readline of numbers
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split numbers
    nums = input.split_whitespace();


    // do operations on positive nums to find answer
    for num in nums
        {
            // get num val
            let mut temp: i32 = num.parse::<i32>().expect("Convert to number = Failed");
            
            //  if positive add 2
            if temp >= 0
                {
                    temp += 2;
                    res.push_str(&temp.to_string());
                    res.push_str(" ");
                }

            else
                {
                    res.push_str(&temp.to_string());
                    res.push_str(" ");
                }

        }
        

    println!("{}", res);


}