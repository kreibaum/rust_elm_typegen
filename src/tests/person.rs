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

impl std::fmt::Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Animal {{ age: {}, pet_name: {} }}",
            self.age, self.pet_name
        )
    }
}

impl Animal {
    #[allow(dead_code)]
    fn moo(&self) {
        println!("Moo!");
    }
}
