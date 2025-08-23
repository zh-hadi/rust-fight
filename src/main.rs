fn main()
{
    println!("hadi");
    let ar: [i32; 4] = [10, 20, 30, 40];
    first_element(ar);
    println!("{}", ar[0]);
}

fn first_element(ar: [i32; 4])
{
    println!("{}", ar[0]);
}