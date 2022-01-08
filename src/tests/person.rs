use super::ElmExport;

#[allow(dead_code)]
struct Person {
    age: u32,
    surname: String,
}

impl ElmExport for Person {}

#[allow(dead_code)]
struct Animal {
    age: u32,
    pet_name: String,
}
