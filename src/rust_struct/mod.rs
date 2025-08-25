#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    status: bool
}


pub fn learn_struct()
{
    println!("learning struct-----------");
    let  user1 = User {
        name: String::from("hadiuzzaman"),
        age: 25,
        status: true
    };

    let user2 = &user1;

    println!("user2 is: {:?}", user2);
    println!("user1 is: {:?}", user1);
}