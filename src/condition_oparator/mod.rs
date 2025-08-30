pub fn load()
{
    match_and_if_learn();
    // println!("hello world");
}


#[allow(dead_code)]
fn match_and_if_learn()
{
    let a = 120;
    let b = 30;

    let x = if a < b {
        format!("a = {} is less then then b = {}", a, b)
    }else {
        format!("a = {} is greater then b = {}", a, b)
    };

    println!("{}", x);

   let r =  match a {
        n if n < 10 => format!("this number is less then 10"),
        n if n > 10 && n < 100 => format!("this number is greather then 10 and less then 100"),
        _ => format!("this number is {} not match", a)
    };

    println!("{}", r);
}