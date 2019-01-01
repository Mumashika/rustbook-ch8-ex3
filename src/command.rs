#[derive(Debug)]
pub enum Command {
    Add(String, String),
    ListDepartment(String),
    ListCompany,
    Exit,
}

impl Command {
    pub fn from(command: &str) -> Option<Command> {
        let lowercase_command = command.to_lowercase();
        if lowercase_command.starts_with("exit") {
            return Some(Command::Exit);
        }

        if lowercase_command.starts_with("list company") {
            return Some(Command::ListCompany);
        }

        let mut words = command.split_whitespace();
        if lowercase_command.starts_with("add") {
            let person = words.nth(1).unwrap().to_string();
            let department = words.last().unwrap().to_string();
            return Some(Command::Add(person, department));
        }

        if lowercase_command.starts_with("list department") {
            let department = words.last().unwrap().to_string();
            return Some(Command::ListDepartment(department));
        }

        None
    }
}
