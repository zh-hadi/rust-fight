pub fn print_array()
{
    let mut  ar = vec![10, 20, 30, 40];
    ar.push(50);
    first_element(&mut ar);
    println!("{:?}", ar);

    array_brower_leaning();
}

fn first_element( ar: &mut Vec<i32>)
{
    ar[0] = 02;

    let mut arr = vec![1, 2, 3];
    let arr2 =  &mut arr;
    println!("{:?}", arr2);
    println!("{:?}", arr);
   
}

fn array_brower_leaning()
{
   let mut ar = vec![11, 22, 33];
   let ar2 = &mut ar;
   ar2[0] = 111;
   println!("{:?}", ar2);
   println!("{:?}", ar);
}