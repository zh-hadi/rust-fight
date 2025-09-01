use std::collections::HashMap;

pub fn hash_map_load()
{
    hash_map_first();
}

fn hash_map_first()
{
    let topic = "HashMap";
    println!("topic is: {topic}");

    let mut data = HashMap::new();

    data.insert("hadi", 2001);
    data.insert("babor", 2000);

    for (key, value) in &data {
        println!("key: {key}, value: {value}");
    }

}