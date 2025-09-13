#[allow(dead_code)]
enum TypePerson {
    Student,
    Teacher
}

pub fn enumaration_load()
{
    let user = TypePerson::Student;

    match user {
        TypePerson::Student => println!("this is a student"),
        _  => println!("this is others person")
    }


}