fn main()
{
    // import input output library
    use std::io;


    // import splitwhitespace object
    use std::str::SplitWhitespace;


    

    // var to hold res [odd number count, even number count]
    let mut res: [i16; 2] = [0, 0];


    // var to hold input
    let mut input: String = String::new();

    // var to hold split numbers
    let mut nums: SplitWhitespace = "".split_whitespace();

    // read numbers in and assign val to input var
    io::stdin().read_line(&mut input).expect("Readline = failed");


    // split input and assign val to nums var
    nums = input.split_whitespace();


    //lesliecharleyhorse
    // for loop to loop thru numbers to convert then to int
    for num in nums
        {
            // temp var to hold value of int
            let mut temp_val: i64 = 0;


            // assign converted value to temp val
            temp_val = num.parse::<i64>().expect("Convert to i64 = Failed");


            // see if odd or even and add to res
            if temp_val % 2 == 0
                {res[1] += 1;}
            else
                {res[0] += 1;}

        }



    // result
    if res[0] > 0 && res[1] > 0
        {
            println!("Yes");
        }
    else
        {println!("No");}


}


        