
mod my_info;


fn main()
{
    my_info::print_name();
    my_info::details::print_age(44);
    my_info::details::birth_date::birth_date();


    let mut  ar = vec![10, 20, 30, 40];
    ar.push(50);
    first_element(&mut ar);
    println!("{:?}", ar);
}

fn first_element( ar: &mut Vec<i32>)
{
    ar[0] = 02;

    let mut arr = vec![1, 2, 3];
    let arr2 =  &mut arr;
    println!("{:?}", arr);
    println!("{:?}", arr2);
   
}