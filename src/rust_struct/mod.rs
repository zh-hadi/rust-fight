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

    let users = vec![user1.clone(), user1.clone(), user1.clone()];

    let first_user = users[0].clone();

    println!("{:?}", users);
    println!("{:?}", user1);
    println!("{:?}", first_user);
}

// summery this lesson
// --------------------
// 