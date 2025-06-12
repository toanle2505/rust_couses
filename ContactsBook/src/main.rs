use std::io;
// This is a simple contact book application in Rust.
#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: String,  
}

impl Contact {
    fn new(name: String, phone: String, email: String) -> Contact {
        Contact {
            name: name,
            phone: phone,
            email: email,
        }
    }

    fn display(&self) {
        println!("Name: {}, Phone: {}, Email: {}", self.name, self.phone, self.email);
    }
}

fn main() {
    let mut contacts : Vec<Contact> = Vec::new();
    loop {
        println!("Contact Book Menu:");
        println!("1. Add Contact");
        println!("2. View Contacts");
        println!("3. Exit");
        println!("Please select an option:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
               add_contact(&mut contacts);
            },
            2 => {
                view_contacts(&contacts);
            },
            3 => {
                println!("Exiting the contact book.");
                break;
            },
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn add_contact(contacts: &mut Vec<Contact>) {
    let mut name = String::new();
    let mut phone = String::new();
    let mut email = String::new();

    println!("Enter name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Enter phone:");
    io::stdin().read_line(&mut phone).expect("Failed to read line");
    println!("Enter email:");
    io::stdin().read_line(&mut email).expect("Failed to read line");

    let contact = Contact::new(name.trim().to_string(), phone.trim().to_string(), email.trim().to_string());
    contacts.push(contact);
}

fn view_contacts(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        println!("No contacts available.");
    } else {
        for contact in contacts {
            contact.display();
        }
    }
}