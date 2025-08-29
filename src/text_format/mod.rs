
pub fn print_text()
{
    let age = 20;
    let mut name = "hadi".to_string();
    let _ = name;
    name = "Hadiuzzaman".to_string();
    
    let info = format!("Name: {}, Age: {}", name, age);
    println!("{}", info);
    println!("length: {}", info.len());
    println!("-------------------------------");

    let number = 255;
    println!("number in decimal: {}", number);
    println!("number in binary: {:b}", number);
    println!("number in Hexadecimal: {:X}", number);
    println!("number in Octal: {:o}", number);


}