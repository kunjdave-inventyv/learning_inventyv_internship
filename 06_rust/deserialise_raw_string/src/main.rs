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
    fn get_info(&self) -> String {
        format!(
            "Person Info:\nName: {}\nAge: {}\nEmail: {}\nPhone: {}",
            self.name, self.age, self.contact.email, self.contact.phone
        )
    }
}

fn main() {
    let raw_json = r#"
    {
        "name": "kunj",
        "age": 20,
        "contact": {
            "email": "kunjdave694@gmail.com",
            "phone": "8320947770"
        }
    }
    "#;

    let person: Person = serde_json::from_str(raw_json).unwrap();

    println!("Deserialized from raw JSON:");
    println!("{}", person.get_info());
}
