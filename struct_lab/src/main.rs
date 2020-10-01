#[derive(Debug)]
struct Sa {
    name: Option<String>,
    key: String,
}

impl Sa {
    fn get_name(&self) -> Option<String> {
        self.name.as_ref().map(|s| format!("sa::{}", s))
    }
    fn get_key(&self) -> String {
        format!("sa::{}", self.key)
    }
}

#[derive(Debug)]
struct Sb {
    name: String,
    key: String,
    name2: String,
    key2: String,
}

impl Sb {
    fn new(sa: &Sa) -> Sb {
        let n2 = sa.get_name().unwrap_or("none".to_string());
        Sb {
            name: "test2".to_string(),
            key: "value2".to_string(),
            name2: n2,
            key2: sa.get_key(),
        }
    }
}

fn main() {
    let sa = Sa {
        name: Some("test".to_string()),
        key: "value".to_string(),
    };

    let sb = Sb::new(&sa);
    println!("{:#?}", sb);
}
