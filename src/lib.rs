use std::collections::HashMap;

use syn::spanned::Spanned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeGenError {
    #[error("{0}")]
    Syn(#[from] syn::Error),
    #[error("Unnamed structs are not supported")]
    UnnamedStructsNotSupported,
}

pub type Result<T> = std::result::Result<T, TypeGenError>;

/// Marker trait for exported types.
pub trait ElmExport {}

// TODO: Handle snake_case -> camelCase conversion
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

pub struct ElmFile {
    name: String,
    structs: Vec<ElmStruct>,
    enums: Vec<ElmEnum>,
}

// Float,
// Char,
// Bool,
// Dict(Box<ElmType>, Box<ElmType>),
// Maybe(Box<ElmType>),
// Result(Box<ElmType>, Box<ElmType>),
#[derive(Debug, Clone)]
pub enum ElmType {
    Int,
    String,
    List(Box<ElmType>),
    NamedType(Identifier), // No generics yet.
}

#[derive(Debug, Clone)]
pub struct ElmStruct {
    name: Identifier,
    fields: Vec<(Identifier, ElmType)>,
}

#[derive(Debug, Clone)]
pub struct ElmEnum {
    name: Identifier,
    variants: Vec<ElmEnumVariant>,
}

#[derive(Debug, Clone)]
struct ElmEnumVariant {
    name: Identifier,
    fields: Vec<ElmType>,
}

const INT_IDENTIFIERS: [&str; 10] = [
    "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize",
];

impl ElmType {
    #[must_use]
    pub fn type_ref(&self) -> String {
        match self {
            ElmType::Int => "Int".to_string(),
            ElmType::String => "String".to_string(),
            ElmType::List(t) => format!("List ({})", t.type_ref()),
            ElmType::NamedType(name) => name.0.clone(),
        }
    }

    #[must_use]
    pub fn decoder_ref(&self) -> String {
        match self {
            ElmType::Int => "Json.Decode.int".to_string(),
            ElmType::String => "Json.Decode.string".to_string(),
            ElmType::List(t) => format!("Json.Decode.list ({})", t.decoder_ref()),
            ElmType::NamedType(name) => format!("decode{}", name.0),
        }
    }

    #[must_use]
    pub fn encoder_ref(&self) -> String {
        match self {
            ElmType::Int => "Json.Encode.int".to_string(),
            ElmType::String => "Json.Encode.string".to_string(),
            ElmType::List(t) => format!("Json.Encode.list ({})", t.encoder_ref()),
            ElmType::NamedType(name) => format!("encode{}", name.0),
        }
    }

    fn from_identifier(identifier: Identifier) -> Self {
        if INT_IDENTIFIERS.iter().any(|s| *s == identifier.0) {
            ElmType::Int
        } else if identifier.0 == "String" {
            ElmType::String
        } else {
            ElmType::NamedType(identifier)
        }
    }
}

impl ElmFile {
    pub fn generate_file_content(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("module {} exposing (..)\n\n\n", self.name));
        result.push_str("import Json.Decode\n");
        result.push_str("import Json.Encode\n");
        result.push_str("import Json.Decode.Pipeline\n");
        for struct_ in &self.structs {
            result.push('\n');
            result.push_str(&struct_.type_def());
            result.push('\n');
            result.push_str(&struct_.encoder_def());
            result.push('\n');
            result.push_str(&struct_.decoder_def());
        }
        for enum_ in &self.enums {
            result.push('\n');
            result.push_str(&enum_.type_def());
            result.push('\n');
            result.push_str(&enum_.encoder_def());
            result.push('\n');
            result.push_str(&enum_.decoder_def());
        }
        result
    }
}

impl ElmStruct {
    #[must_use]
    pub fn type_ref(&self) -> String {
        self.name.0.clone()
    }

    #[must_use]
    pub fn type_def(&self) -> String {
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

    #[must_use]
    pub fn decoder_ref(&self) -> String {
        // Outputs something like decodePerson
        format!("decode{}", self.name.0)
    }

    #[must_use]
    pub fn decoder_def(&self) -> String {
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

    #[must_use]
    pub fn encoder_ref(&self) -> String {
        format!("encode{}", self.name.0)
    }

    #[must_use]
    pub fn encoder_def(&self) -> String {
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

impl ElmEnum {
    pub fn type_def(&self) -> String {
        // Outputs something like:
        // type Message
        //     = PrimaryVariant
        //     | SecondaryVariant Int String
        //     | ThirdVariant

        let mut output = format!("type {}\n", self.name.0);
        let mut is_first = true;
        for variant in &self.variants {
            if is_first {
                output.push_str("    = ");
                is_first = false;
            } else {
                output.push_str("    | ");
            }
            output.push_str(&variant.name.0);
            for field in &variant.fields {
                output.push(' ');
                output.push_str(&field.type_ref());
            }
            output.push('\n');
        }
        output
    }

    fn encoder_def(&self) -> String {
        let mut output = format!(
            "encode{} : {} -> Json.Encode.Value\n",
            self.name.0, self.name.0
        );
        let this = self.name.0.to_lowercase();
        output.push_str(&format!("encode{} {} =\n", self.name.0, this));
        output.push_str(&format!("    case {} of\n", this));

        for variant in &self.variants {
            if variant.fields.is_empty() {
                output.push_str(&format!("        {} ->\n", variant.name.0));
                output.push_str(&format!(
                    "            Json.Encode.string \"{}\"\n\n",
                    variant.name.0
                ));
            } else if variant.fields.len() == 1 {
                let field = variant.fields.first().unwrap();
                output.push_str(&format!("        {} x ->\n", variant.name.0));
                output.push_str("            Json.Encode.object\n");
                output.push_str(&format!(
                    "                [ ( \"{}\", {} x )\n",
                    variant.name.0,
                    field.encoder_ref()
                ));
                output.push_str("                ]\n\n");
            } else {
                output.push_str(&format!("        {} ", variant.name.0));
                for (i, _) in variant.fields.iter().enumerate() {
                    output.push_str(&format!("x{} ", i));
                }
                output.push_str("->\n");
                output.push_str("            Json.Encode.object\n");
                output.push_str(&format!("                [ ( \"{}\"\n", variant.name.0));
                output.push_str("                  , Json.Encode.list (\\v -> v)\n");
                let mut is_first = true;
                for (i, field) in variant.fields.iter().enumerate() {
                    if is_first {
                        output.push_str("                        [ ");
                        is_first = false;
                    } else {
                        output.push_str("                        , ");
                    }
                    output.push_str(&format!("{} x{}\n", field.encoder_ref(), i));
                }
                output.push_str("                        ]\n");
                output.push_str("                  )\n");
                output.push_str("                ]\n\n");
            }
        }

        output
    }

    fn decoder_def(&self) -> String {
        let mut output = format!(
            "decode{} : Json.Decode.Decoder {}\n",
            self.name.0, self.name.0
        );

        // General decoder that collects all variant decoders
        output.push_str(&format!("decode{} =\n", self.name.0));
        output.push_str("    Json.Decode.oneOf\n");
        let mut is_first = true;
        for variant in &self.variants {
            if is_first {
                output.push_str("        [ ");
                is_first = false;
            } else {
                output.push_str("        , ");
            }
            output.push_str(&format!("decode{}{}\n", self.name.0, variant.name.0));
        }
        output.push_str("        ]\n");

        // Variant decoders
        for variant in &self.variants {
            output.push_str("\n\n");
            output.push_str(&format!(
                "decode{}{} : Json.Decode.Decoder {}\n",
                self.name.0, variant.name.0, self.name.0
            ));
            output.push_str(&format!("decode{}{} =\n", self.name.0, variant.name.0));
            if variant.fields.is_empty() {
                // TODO: All those variants should be joined together for performance.
                output.push_str("    Json.Decode.andThen\n");
                output.push_str("        (\\str ->\n");
                output.push_str("            case str of\n");
                output.push_str(&format!("                \"{}\" ->\n", variant.name.0));
                output.push_str(&format!(
                    "                    Json.Decode.succeed {}\n\n",
                    variant.name.0
                ));
                output.push_str("                _ ->\n");
                output.push_str(&format!(
                    "                    Json.Decode.fail \"Expected variant {}\"\n",
                    variant.name.0
                ));
                output.push_str("        )\n");
                output.push_str("        Json.Decode.string\n")
            } else if variant.fields.len() == 1 {
                output.push_str(&format!("    Json.Decode.succeed {}\n", variant.name.0));
                let field = variant.fields.first().unwrap();
                output.push_str(&format!(
                    "        |> Json.Decode.Pipeline.required \"{}\" {}\n",
                    variant.name.0,
                    field.decoder_ref()
                ));
            } else {
                output.push_str(&format!("    Json.Decode.succeed {}\n", variant.name.0));
                for (i, ty) in variant.fields.iter().enumerate() {
                    // |> Json.Decode.Pipeline.custom
                    //     (Json.Decode.field "Compare" (Json.Decode.index 0 Json.Decode.int))
                    output.push_str("        |> Json.Decode.Pipeline.custom \n");
                    output.push_str(&format!(
                        "            (Json.Decode.field \"{}\" (Json.Decode.index {} {}))\n",
                        variant.name.0,
                        i,
                        ty.decoder_ref()
                    ));
                }
            }
        }

        output
    }
}

#[derive(Debug)]
pub struct RustFile {
    /// Indicates which types are requested for the export. Any types that are
    /// referenced by this will also be included in the output but are not
    /// the main export types.
    pub main_export_types: Vec<Identifier>,
    /// All structs we have seen in the file. Maybe not all of them need to be
    /// turned into elm structs.
    pub all_structs: HashMap<Identifier, ElmStruct>,
    /// All structs that are exported into the target elm file
    pub export_structs: Vec<ElmStruct>,
    /// All enums we have seen in the file.
    pub all_enums: HashMap<Identifier, ElmEnum>,
    /// All enums that are exported into the target elm file
    pub export_enums: Vec<ElmEnum>,
}

impl RustFile {
    pub fn parse(ast: &syn::File) -> Result<RustFile> {
        let main_export_types = discover_export_types(ast)?;
        let mut main_export_enums = Vec::new();

        let all_structs = find_all_structs(ast)?;

        let mut export_structs: Vec<ElmStruct> = vec![];
        for identifier in &main_export_types {
            if let Some(struct_) = all_structs.get(identifier) {
                export_structs.push(struct_.clone());
            } else {
                main_export_enums.push(identifier.clone());
            }
        }

        let all_enums = find_all_enums(ast)?;

        let mut export_enums: Vec<ElmEnum> = vec![];
        for identifier in &main_export_enums {
            if let Some(enum_) = all_enums.get(identifier) {
                export_enums.push(enum_.clone());
            } else {
                panic!("Enum/Struct {:?} not found", identifier);
            }
        }

        Ok(RustFile {
            main_export_types,
            all_structs,
            export_structs,
            all_enums,
            export_enums,
        })
    }
}

fn find_all_enums(ast: &syn::File) -> Result<HashMap<Identifier, ElmEnum>> {
    let mut result = HashMap::new();

    for item in &ast.items {
        if let syn::Item::Enum(item_enum) = item {
            let identifier = Identifier(item_enum.ident.to_string());
            let mut variants = vec![];
            for variant in &item_enum.variants {
                let var_ident = Identifier(variant.ident.to_string());
                let mut fields = vec![];
                for field in variant.fields.iter() {
                    let ty = elm_type_from_type(&field.ty)?;
                    fields.push(ty);
                }
                variants.push(ElmEnumVariant {
                    name: var_ident,
                    fields,
                });
            }
            result.insert(
                identifier.clone(),
                ElmEnum {
                    name: identifier,
                    variants,
                },
            );
        }
    }

    Ok(result)
}

fn discover_export_types(ast: &syn::File) -> Result<Vec<Identifier>> {
    let mut main_export_types = Vec::new();
    for item in &ast.items {
        if let syn::Item::Impl(item_impl) = item {
            if let Some((_, item_impl_trait, _)) = &item_impl.trait_ {
                let trait_ident = last_path(item_impl_trait)?;

                if trait_ident.0 == "ElmExport" {
                    if let syn::Type::Path(type_path) = item_impl.self_ty.as_ref() {
                        let type_for_export = simple_path(&type_path.path)?;
                        main_export_types.push(type_for_export);
                    }
                }
            }
        }
    }
    Ok(main_export_types)
}

fn find_all_structs(ast: &syn::File) -> Result<HashMap<Identifier, ElmStruct>> {
    let mut result = HashMap::new();

    for item in &ast.items {
        if let syn::Item::Struct(item_struct) = item {
            let identifier = Identifier(item_struct.ident.to_string());
            match &item_struct.fields {
                syn::Fields::Named(fields) => {
                    result.insert(identifier.clone(), extract_elm_struct(identifier, fields)?);
                }
                syn::Fields::Unnamed(_) => return Err(TypeGenError::UnnamedStructsNotSupported),
                syn::Fields::Unit => return Err(TypeGenError::UnnamedStructsNotSupported),
            }
        }
    }

    Ok(result)
}

fn extract_elm_struct(identifier: Identifier, fields: &syn::FieldsNamed) -> Result<ElmStruct> {
    let mut result = ElmStruct {
        name: identifier,
        fields: vec![],
    };
    for field in &fields.named {
        let ident = Identifier(field.ident.as_ref().unwrap().to_string());
        let ty = elm_type_from_type(&field.ty)?;
        result.fields.push((ident, ty));
    }
    Ok(result)
}

fn elm_type_from_type(ty: &syn::Type) -> Result<ElmType> {
    let ty = match &ty {
        syn::Type::Array(_) => todo!("Array missing"),
        syn::Type::BareFn(_) => todo!("BareFn missing"),
        syn::Type::Group(_) => todo!("Group missing"),
        syn::Type::ImplTrait(_) => todo!("ImplTrait missing"),
        syn::Type::Infer(_) => todo!("Infer missing"),
        syn::Type::Macro(_) => todo!("Macro missing"),
        syn::Type::Never(_) => todo!("Never missing"),
        syn::Type::Paren(_) => todo!("Paren missing"),
        syn::Type::Path(type_path) => ElmType::from_identifier(simple_path(&type_path.path)?),
        syn::Type::Ptr(_) => todo!("Ptr missing"),
        syn::Type::Reference(_) => todo!("Reference missing"),
        syn::Type::Slice(_) => todo!("Slice missing"),
        syn::Type::TraitObject(_) => todo!("TraitObject missing"),
        syn::Type::Tuple(_) => todo!("Tuple missing"),
        syn::Type::Verbatim(_) => todo!("Verbatim missing"),
        syn::Type::__TestExhaustive(_) => todo!("__TestExhaustive missing"),
    };
    Ok(ty)
}

fn simple_path(path: &syn::Path) -> Result<Identifier> {
    if path.segments.len() != 1 {
        return Err(syn::Error::new(path.span(), "Only simple paths are supported").into());
    }
    Ok(Identifier(path.segments.first().unwrap().ident.to_string()))
}

fn last_path(path: &syn::Path) -> Result<Identifier> {
    let last = path.segments.last();
    if let Some(last) = last {
        Ok(Identifier(last.ident.to_string()))
    } else {
        Err(syn::Error::new(path.span(), "There is nothing in the path.").into())
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use std::fs::File;
    use std::io::Read;

    mod message;
    mod person;

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

    #[test]
    /// Test for the person.rs file
    fn test_person_file() -> Result<()> {
        let mut rust_file = File::open("src/tests/person.rs").expect("Failed to open file");
        let mut rust_file_content = String::new();
        rust_file
            .read_to_string(&mut rust_file_content)
            .expect("Failed to read file");

        let ast = syn::parse_file(&rust_file_content).expect("Failed to parse file");

        let rust_file = RustFile::parse(&ast)?;
        assert_eq!(1, rust_file.main_export_types.len());
        assert_eq!(
            rust_file.main_export_types[0],
            Identifier("Person".to_string())
        );

        assert_eq!(rust_file.export_structs.len(), 1);
        assert_eq!(
            rust_file.export_structs.get(0).unwrap().type_def(),
            indoc! {"
            type alias Person =
                { age : Int
                , surname : String
                }
            "
            }
        );

        let mut elm_file = File::open("src/tests/Person.elm").expect("Failed to open file");
        let mut elm_file_content = String::new();
        elm_file
            .read_to_string(&mut elm_file_content)
            .expect("Failed to read file");

        let elm_file_object = ElmFile {
            name: "Person".to_string(),
            structs: rust_file.export_structs,
            enums: rust_file.export_enums,
        };

        assert_eq!(elm_file_object.generate_file_content(), elm_file_content);

        Ok(())
    }

    /// Test for enum parsing
    #[test]
    fn test_message_file() {
        let rust_file = parse_rust_file_for_test("src/tests/message.rs");
        let elm_file_content = read_file_for_test("src/tests/Message.elm");

        let elm_file_object = ElmFile {
            name: "Message".to_string(),
            structs: rust_file.export_structs,
            enums: rust_file.export_enums,
        };

        assert_eq!(elm_file_object.generate_file_content(), elm_file_content);
    }

    fn read_file_for_test(path: &str) -> String {
        let mut rust_file = File::open(path).expect("Failed to open file");
        let mut rust_file_content = String::new();
        rust_file
            .read_to_string(&mut rust_file_content)
            .expect("Failed to read file");
        rust_file_content
    }

    fn parse_rust_file_for_test(path: &str) -> RustFile {
        let ast = syn::parse_file(&read_file_for_test(path)).expect("Failed to parse file");
        RustFile::parse(&ast).expect("Failed to parse file")
    }

    #[test]
    fn test_various_primitives() {
        let rust_file = parse_rust_file_for_test("src/tests/primitives.rs");
        let elm_file_content = read_file_for_test("src/tests/Primitives.elm");

        let elm_file_object = ElmFile {
            name: "Message".to_string(),
            structs: rust_file.export_structs,
            enums: rust_file.export_enums,
        };

        assert_eq!(elm_file_object.generate_file_content(), elm_file_content);
    }

    #[test]
    fn test_named_type_reference() {
        let rust_file = parse_rust_file_for_test("src/tests/reference.rs");
        let elm_file_content = read_file_for_test("src/tests/Reference.elm");

        let elm_file_object = ElmFile {
            name: "Message".to_string(),
            structs: rust_file.export_structs,
            enums: rust_file.export_enums,
        };

        assert_eq!(elm_file_object.generate_file_content(), elm_file_content);
    }
}
