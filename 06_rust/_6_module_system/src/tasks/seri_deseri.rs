use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ContactInfo {
    email: String,
    phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    contact: ContactInfo,
}

impl Person {
    fn new(name: String, age: u32, email: String, phone: String) -> Self {
        Person {
            name,
            age,
            contact: ContactInfo { email, phone },
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }

    fn get_email(&self) -> &str {
        &self.contact.email
    }

    fn get_phone(&self) -> &str {
        &self.contact.phone
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    fn set_email(&mut self, email: String) {
        self.contact.email = email;
    }

    fn set_phone(&mut self, phone: String) {
        self.contact.phone = phone;
    }

    fn get_info(&self) -> String {
        format!(
            "Person Info:\nName: {}\nAge: {}\nEmail: {}\nPhone: {}",
            self.name, self.age, self.contact.email, self.contact.phone
        )
    }
}

pub fn start() {
    let mut p = Person::new(
        "kunj".to_string(),
        20,
        "kunjdave694@gmail.com".to_string(),
        "8320947770".to_string(),
    );

    println!("Name: {}", p.get_name());
    println!("Age: {}", p.get_age());
    println!();
    println!("{}", p.get_info());
    println!();
    println!();
    println!();
    
    p.set_age(30);
    p.set_email("kunj@gmail.com".to_string());
    p.set_phone("+91 8320947770".to_string());

    println!("{}", p.get_info());
    println!();
    println!();

    let json_string = serde_json::to_string(&p).unwrap();
    println!("Serialized to JSON:");
    println!("{}", json_string);
    println!();
    println!();

    let person_from_json: Person = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized from JSON:");
    println!("{}", person_from_json.get_info());
}