use std::io;

mod command;
use crate::command::Command;

mod company;
use crate::company::Company;

fn main() {
    println!("Usage examples:");
    println!("* Add Sally to Engineering");
    println!("* List department Engineering ");
    println!("* List company");
    println!("* Exit");

    let mut company = Company::new();

    loop {
        let mut line = String::new();
        println!("Please input your command:");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        let command = Command::from(&line);

        match command {
            Some(Command::Add(person, department)) => company.add(person, department),
            Some(Command::ListDepartment(department)) => company.list_department(&department),
            Some(Command::ListCompany) => company.list_company(),
            Some(Command::Exit) => break,
            None => (),
        }
    }
}
