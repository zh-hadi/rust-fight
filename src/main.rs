// fn main()
// {
//     println!("hadi");
//     let mut ar: [i32; 4] = [10, 20, 30, 40];
//     first_element(&mut ar);
//     println!("{}", ar[0]);
// }

// fn first_element(ar: &mut [i32])
// {
//     ar[0] = 100;
//     println!("{}", ar[0]);
// }


// now using vector data 
fn main()
{
    let mut  ar = vec![10, 20, 30, 40];
    ar.push(50);
    first_element(ar.clone());
    println!("{:?}", ar);
}

fn first_element(ar: Vec<i32>)
{
    println!("{}", ar[0]);
}