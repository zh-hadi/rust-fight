#![allow(dead_code)]
pub fn print_array()
{
    let mut  ar = vec![10, 20, 30, 40];
    ar.push(50);

    let mut ar2 = data_multiply_by_2(ar.clone());
    let ar3 = data_multiply_by_2_same(&mut ar2);
    println!("{:?}", ar);
    println!("{:?}", ar2);

    // for (index, value) in ar.iter_mut().enumerate() {
    //     *value = *value * 2; 
    //     println!("index[{}] = {}", index, value);
    // }

    // for i in 0..ar.len() {
    //     ar[i] = ar[i] * 2;
    //     println!("{}", i);
    // }

    // println!("{:?}", ar);



    // first_element(&mut ar);
    // println!("{:?}", ar);

    // array_brower_leaning();
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


// array data double using new array function 
fn data_multiply_by_2(mut ar: Vec<i32>)-> Vec<i32>
{
    for i in 0..ar.len()  {
        ar[i] *= 2;
    }
    return ar;
}
// array data dobule in same array function 
fn data_multiply_by_2_same(ar: &mut Vec<i32>)
{
    for i in 0..ar.len(){
        ar[i] *= 2;
    }
}