use std::cell::RefCell;

struct ContactInfo {
    email: String,
    phone: String,
}

struct PersonData {
    name: String,
    age: u32,
    contact: ContactInfo,
}

struct Person {
    data: RefCell<PersonData>,
}

impl Person {
    fn new(name: String, age: u32, email: String, phone: String) -> Self {
        Person {
            data: RefCell::new(PersonData {
                name,
                age,
                contact: ContactInfo { email, phone },
            }),
        }
    }

    fn set_name(&self, name: String) {
        self.data.borrow_mut().name = name;
        self.print_state();
    }

    fn set_age(&self, age: u32) {
        self.data.borrow_mut().age = age;  
        self.print_state();
    }

    fn set_email(&self, email: String) {
        self.data.borrow_mut().contact.email = email;  
        self.print_state();
    }

    fn set_phone(&self, phone: String) {
        self.data.borrow_mut().contact.phone = phone;  
        self.print_state();
    }

    fn update_whole(&self, name: String, age: u32, email: String, phone: String) {
        *self.data.borrow_mut() = PersonData {
            name,
            age,
            contact: ContactInfo { email, phone },
        };
        self.print_state();
    }

    fn print_state(&self) {
        let data = self.data.borrow();
        println!("=== First instance State ===");
        println!("Name: {}", &data.name);
        println!("Age: {}", &data.age);
        println!("Email: {}", &data.contact.email);
        println!("Phone: {}", &data.contact.phone);
        println!("====================\n");
        println!("=== Second Instance State ===");
        println!("Name: {}", data.name);
        println!("Age: {}", data.age);
        println!("Email: {}", data.contact.email);
        println!("Phone: {}", data.contact.phone);
        println!("====================\n");
    }
}


pub fn start() {
    let p1 = Person::new(
        "kunj".to_string(),
        20,
        "kunjdave694@gmail.com".to_string(),
        "8320947770".to_string(),
    );

    println!("Initial state:");
    p1.print_state();

    let p2 = &p1;

    p2.set_name("Kunj Dave".to_string());
    p2.set_age(21);
    p2.set_email("kunjdave.updated@gmail.com".to_string());
    p2.set_phone("9876543210".to_string());

    println!("Final state (accessed through p2):");
    p2.print_state();

    println!("Using update_whole method:");
    p2.update_whole(
        "New Name".to_string(),
        25,
        "newemail@example.com".to_string(),
        "1111111111".to_string(),
    );
}