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
mod my_info;

// now using vector data 
fn main()
{
    my_info::my_info();
    let mut  ar = vec![10, 20, 30, 40];
    ar.push(50);
    first_element(&mut ar);
    println!("{:?}", ar);
}

fn first_element( ar: &mut Vec<i32>)
{
    ar[0] = 02;
    // println!("{}", ar[0]);
    // println!("{:?}", ar);
    // println!("{:?}", ar);
}