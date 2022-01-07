use std::fmt::Display;

// Float,
// Char,
// Bool,
// Dict(Box<ElmType>, Box<ElmType>),
// Maybe(Box<ElmType>),
// Result(Box<ElmType>, Box<ElmType>),
enum ElmType {
    Int,
    String,
    List(Box<ElmType>),
}

impl ElmType {
    fn elm_type(&self) -> String {
        match self {
            ElmType::Int => "Int".to_string(),
            ElmType::String => "String".to_string(),
            ElmType::List(t) => format!("List ({})", t.elm_type()),
        }
    }

    fn decoder(&self) -> String {
        match self {
            ElmType::Int => "Json.Decode.int".to_string(),
            ElmType::String => "Json.Decode.string".to_string(),
            ElmType::List(t) => format!("Json.Decode.list ({})", t.decoder()),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let ty = ElmType::List(Box::new(ElmType::Int));
    assert_eq!(ty.elm_type(), "List (Int)");
    assert_eq!(ty.decoder(), "Json.Decode.list (Json.Decode.int)");
    let ty = ElmType::String;
    assert_eq!(ty.elm_type(), "String");
    assert_eq!(ty.decoder(), "Json.Decode.string");
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_export() {
        let ty = ElmType::Int;
        assert_eq!(ty.elm_type(), "Int");
        assert_eq!(ty.decoder(), "Json.Decode.int");
        let ty = ElmType::List(Box::new(ElmType::Int));
        assert_eq!(ty.elm_type(), "List (Int)");
        assert_eq!(ty.decoder(), "Json.Decode.list (Json.Decode.int)");
        let ty = ElmType::List(Box::new(ElmType::List(Box::new(ElmType::Int))));
        assert_eq!(ty.elm_type(), "List (List (Int))");
        assert_eq!(
            ty.decoder(),
            "Json.Decode.list (Json.Decode.list (Json.Decode.int))"
        );
        let ty = ElmType::String;
        assert_eq!(ty.elm_type(), "String");
        assert_eq!(ty.decoder(), "Json.Decode.string");
    }
}
