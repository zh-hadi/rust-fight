#[derive(Debug, Clone)]
struct User {
    name: String,
    age: i32,
    status: bool
}

impl User {
    fn print_user(&self)
    {
        println!("name: {}, age: {}, status: {}", self.name, self.age, self.status);
    }
}


pub fn learn_struct()
{
    println!("learning struct-----------");
    let  user1 = User {
        name: String::from("hadiuzzaman"),
        age: 25,
        status: true
    };

    let user2 = user1.clone();

    user1.print_user();
    
    println!("user1 is: {:?}", user1);
    println!("user2 is: {:?}", user2);
}

// summery this lesson
// --------------------
// understand ownership deeply if struct mehtod
// i am want refercnce then ownership move 
// this data not avalible any more so clear struct or ther data 