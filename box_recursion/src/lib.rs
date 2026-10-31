#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let worker = Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        });
        self.grade = Some(worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| {
            self.grade = worker.next;
            worker.name
        })
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade.as_ref().map(|w| {
            let role = match w.role {
                Role::CEO => Role::CEO,
                Role::Manager => Role::Manager,
                Role::Worker => Role::Worker,
            };
            (w.name.clone(), role)
        })
    }
}