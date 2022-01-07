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
    fn type_ref(&self) -> String {
        match self {
            ElmType::Int => "Int".to_string(),
            ElmType::String => "String".to_string(),
            ElmType::List(t) => format!("List ({})", t.type_ref()),
        }
    }

    fn decoder_ref(&self) -> String {
        match self {
            ElmType::Int => "Json.Decode.int".to_string(),
            ElmType::String => "Json.Decode.string".to_string(),
            ElmType::List(t) => format!("Json.Decode.list ({})", t.decoder_ref()),
        }
    }

    fn encoder_ref(&self) -> String {
        match self {
            ElmType::Int => "Json.Encode.int".to_string(),
            ElmType::String => "Json.Encode.string".to_string(),
            ElmType::List(t) => format!("Json.Encode.list ({})", t.encoder_ref()),
        }
    }
}

// TODO: Handle snake_case -> camelCase conversion
#[derive(Debug, Clone)]
struct Identifier(String);

struct ElmStruct {
    name: Identifier,
    fields: Vec<(Identifier, ElmType)>,
}

impl ElmStruct {
    fn type_ref(&self) -> String {
        self.name.0.clone()
    }

    fn type_def(&self) -> String {
        // Outputs something like:
        // type alias Person =
        //     { age : Int
        //     , surname : String
        //     }
        let mut output = format!("type alias {} =\n", self.name.0);
        let mut is_first = true;
        for (field, ty) in &self.fields {
            if is_first {
                output.push_str("    { ");
                is_first = false;
            } else {
                output.push_str("    , ");
            }
            output.push_str(&format!("{} : {}\n", field.0, ty.type_ref()));
        }
        output.push_str("    }\n");
        output
    }

    fn decoder_ref(&self) -> String {
        // Outputs something like decodePerson
        format!("decode{}", self.name.0)
    }

    fn decoder_def(&self) -> String {
        // Outputs something like:
        // decodePerson : Json.Decode.Decoder Person
        // decodePerson =
        //     Json.Decode.succeed Person
        //         |> Json.Decode.Pipeline.required "age" Json.Decode.int
        //         |> Json.Decode.Pipeline.required "surname" Json.Decode.string
        let mut output = format!(
            "decode{} : Json.Decode.Decoder {}\n",
            self.name.0, self.name.0
        );
        output.push_str(format!("decode{} =\n", self.name.0).as_str());
        output.push_str(format!("    Json.Decode.succeed {}\n", self.name.0).as_str());
        for (field, ty) in &self.fields {
            output.push_str(
                format!(
                    "        |> Json.Decode.Pipeline.required \"{}\" {}\n",
                    field.0,
                    ty.decoder_ref()
                )
                .as_str(),
            );
        }
        output
    }

    fn encoder_ref(&self) -> String {
        format!("encode{}", self.name.0)
    }

    fn encoder_def(&self) -> String {
        // Outputs something like:
        // encodePerson : Person -> Json.Encode.Value
        // encodePerson person =
        //     Json.Encode.object
        //         [ ( "age", Json.Encode.int person.age )
        //         , ( "surname", Json.Encode.string person.surname )
        //         ]
        let mut output = format!(
            "encode{} : {} -> Json.Encode.Value\n",
            self.name.0, self.name.0
        );
        let this = self.name.0.to_lowercase();
        output.push_str(format!("encode{} {} =\n", self.name.0, this).as_str());
        output.push_str("    Json.Encode.object\n");
        let mut is_first = true;
        for (field, ty) in &self.fields {
            if is_first {
                output.push_str("        [ ");
                is_first = false;
            } else {
                output.push_str("        , ");
            }
            output.push_str(
                format!(
                    "( \"{}\", {} {}.{} )\n",
                    field.0,
                    ty.encoder_ref(),
                    this,
                    field.0
                )
                .as_str(),
            );
        }
        output.push_str("        ]\n");
        output
    }
}

fn main() {
    let ty = ElmStruct {
        name: Identifier("Person".to_string()),
        fields: vec![
            (Identifier("age".to_string()), ElmType::Int),
            (Identifier("surname".to_string()), ElmType::String),
        ],
    };
    println!("{}", ty.type_ref());
    println!("{}", ty.type_def());
    println!("{}", ty.decoder_ref());
    println!("{}", ty.decoder_def());
    println!("{}", ty.encoder_ref());
    println!("{}", ty.encoder_def());
}

// Test module
#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn basic_export() {
        let ty = ElmType::Int;
        assert_eq!(ty.type_ref(), "Int");
        assert_eq!(ty.decoder_ref(), "Json.Decode.int");
        assert_eq!(ty.encoder_ref(), "Json.Encode.int");
        let ty = ElmType::List(Box::new(ElmType::Int));
        assert_eq!(ty.type_ref(), "List (Int)");
        assert_eq!(ty.decoder_ref(), "Json.Decode.list (Json.Decode.int)");
        assert_eq!(ty.encoder_ref(), "Json.Encode.list (Json.Encode.int)");
        let ty = ElmType::List(Box::new(ElmType::List(Box::new(ElmType::Int))));
        assert_eq!(ty.type_ref(), "List (List (Int))");
        assert_eq!(
            ty.decoder_ref(),
            "Json.Decode.list (Json.Decode.list (Json.Decode.int))"
        );
        assert_eq!(
            ty.encoder_ref(),
            "Json.Encode.list (Json.Encode.list (Json.Encode.int))"
        );
        let ty = ElmType::String;
        assert_eq!(ty.type_ref(), "String");
        assert_eq!(ty.decoder_ref(), "Json.Decode.string");
        assert_eq!(ty.encoder_ref(), "Json.Encode.string");
    }

    #[test]
    fn test_struct() {
        let ty = ElmStruct {
            name: Identifier("Person".to_string()),
            fields: vec![
                (Identifier("age".to_string()), ElmType::Int),
                (Identifier("surname".to_string()), ElmType::String),
            ],
        };
        assert_eq!(ty.type_ref(), "Person");
        assert_eq!(
            ty.type_def(),
            "type alias Person =\n    { age : Int\n    , surname : String\n    }\n"
        );
        assert_eq!(ty.decoder_ref(), "decodePerson");
        assert_eq!(
            ty.decoder_def(),
            indoc! {"
                decodePerson : Json.Decode.Decoder Person
                decodePerson =
                    Json.Decode.succeed Person
                        |> Json.Decode.Pipeline.required \"age\" Json.Decode.int
                        |> Json.Decode.Pipeline.required \"surname\" Json.Decode.string
                "
            }
        );
        assert_eq!(ty.encoder_ref(), "encodePerson");
        assert_eq!(
            ty.encoder_def(),
            indoc! {"
                encodePerson : Person -> Json.Encode.Value
                encodePerson person =
                    Json.Encode.object
                        [ ( \"age\", Json.Encode.int person.age )
                        , ( \"surname\", Json.Encode.string person.surname )
                        ]
                "
            }
        );
    }
}
