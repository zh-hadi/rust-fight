use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Person {
    name: String,
    age: i32
}

pub fn initialization(){
    println!("hello world json");
    // struct_to_json();
    read_json_file();
}

#[allow(dead_code)]
fn struct_to_json()
{
    let p1 = Person {
        name: "hadiuzzaman hadi".to_string(),
        age: 25
    };

    let mut all_user = vec![];
    all_user.push(p1.clone());
    all_user.push(p1.clone());

    println!("{:?}", all_user);

    let json_data = serde_json::to_string_pretty(&all_user).unwrap();

    println!("{}", json_data);

    println!("{:?}", p1);

    let mut file = File::create("person.json").expect("file not create");
    file.write_all(json_data.as_bytes()).expect("data not save here");
}

fn read_json_file()
{
    add_person("babor".to_string(), 25);
}

fn add_person(name: String, age: i32)
{
    let p1 = Person{
        name: name,
        age: age
    };

    println!("{:?}", p1);

    let mut file = File::open("person.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    println!("{}", data);

}