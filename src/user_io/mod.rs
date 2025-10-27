use std::io;

pub fn user_io_load()
{
    loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input);
    
        println!("output is : {}", input);
    
    
        let s2: Result<i32, String> = input.trim().parse::<i32>().map_err(|e| e.to_string());
        // println!("{}", s2);
        match s2 {
            Ok(num) => println!("the number is: {}", num),
            Err(e) => println!("error is: {}", e)
        }

        
    }
}