use std::{collections::HashMap, fmt};

#[derive(Debug)]
enum InventoryError {
    Duplicate,
    Invalid,
    Missing
}

trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Debug)]
struct Inventory<'a, T>
where
    T: DisplayItem + Clone,
{
    data: &'a mut HashMap<String, T>,
}

impl<'a, T> Inventory<'a, T>
where
    T: DisplayItem + Clone,
{
    fn new(data: &'a mut HashMap<String, T>) -> Self {
        Inventory {
            data
        }
    }

    fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::Missing);
        }
        if !id.trim().starts_with('P') {
            return Err(InventoryError::Invalid);
        }
        if self.data.contains_key(&id) {
            return Err(InventoryError::Duplicate);
        }

        self.data.insert(id, item);
        Ok(())
    }

    fn display_all(&self) -> String {
        if self.data.is_empty() {
            return "Inventory is empty".to_string();
        }

        self.data
            .iter()
            .map(|(id, item)| {
                format!("ID: {}\n{}\n\n", id, item.display())
            })
            .collect()
    }
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::Duplicate => write!(f, "Duplicate ID error"),
            InventoryError::Invalid => write!(f, "Invalid ID error : It should start with P "),
            InventoryError::Missing => write!(f, "Missing ID error"),
        }
    }
}

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("Product: {}, Price: ${}", self.name, self.price)
    }
}

fn main() {
    let mut storage: HashMap<String, Product> = HashMap::new();   // real owner

    let mut inventory = Inventory::new(&mut storage);   
    let p1 = Product {
        name: "Laptop".to_string(),
        price: 1200.0,
    };

    let p2 = Product {
        name: "Mouse".to_string(),
        price: 25.0,
    };


    if let Err(e) = inventory.add_item("P001".to_string(), p2.clone()) {
        println!("Error: {}", e);
    }
    if let Err(e) = inventory.add_item("P002".to_string(), p1.clone()) {
        println!("Error: {}", e);
    }
    println!("{}", inventory.display_all());
}