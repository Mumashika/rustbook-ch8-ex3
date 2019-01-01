use std::collections::HashMap;

#[derive(Debug)]
pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    pub fn add(&mut self, person: String, department: String) {
        let department = self.departments.entry(department).or_insert(vec![]);
        department.push(person);
    }

    pub fn list_department(&self, department_name: &str) {
        if let Some(department) = self.departments.get(department_name) {
            let mut department = department.clone();
            department.sort();

            println!("Employees in department {}:", department_name);
            for person in department {
                println!("* {}", person);
            }
        } else {
            println!("Departmernt {} does not exist", department_name);
        }
    }

    pub fn list_company(&self) {
        for department in self.departments.keys() {
            self.list_department(&department);
        }
    }
}
