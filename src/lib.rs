use std::collections::HashMap;

use syn::{ext::IdentExt, spanned::Spanned};
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

// Float,
// Char,
// Bool,
// Dict(Box<ElmType>, Box<ElmType>),
// Maybe(Box<ElmType>),
// Result(Box<ElmType>, Box<ElmType>),
#[derive(Debug)]
pub enum ElmType {
    Int,
    String,
    List(Box<ElmType>),
}

impl ElmType {
    #[must_use]
    pub fn type_ref(&self) -> String {
        match self {
            ElmType::Int => "Int".to_string(),
            ElmType::String => "String".to_string(),
            ElmType::List(t) => format!("List ({})", t.type_ref()),
        }
    }

    #[must_use]
    pub fn decoder_ref(&self) -> String {
        match self {
            ElmType::Int => "Json.Decode.int".to_string(),
            ElmType::String => "Json.Decode.string".to_string(),
            ElmType::List(t) => format!("Json.Decode.list ({})", t.decoder_ref()),
        }
    }

    #[must_use]
    pub fn encoder_ref(&self) -> String {
        match self {
            ElmType::Int => "Json.Encode.int".to_string(),
            ElmType::String => "Json.Encode.string".to_string(),
            ElmType::List(t) => format!("Json.Encode.list ({})", t.encoder_ref()),
        }
    }

    fn from_identifier(identifier: Identifier) -> Self {
        if identifier.0 == "u32" {
            ElmType::Int
        } else if identifier.0 == "String" {
            ElmType::String
        } else {
            todo!("identifier not implemented yet.")
        }
    }
}

// TODO: Handle snake_case -> camelCase conversion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(String);

#[derive(Debug)]
pub struct ElmStruct {
    name: Identifier,
    fields: Vec<(Identifier, ElmType)>,
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

#[derive(Debug)]
pub struct RustFile {
    pub main_export_types: Vec<Identifier>,
    pub all_structs: Vec<ElmStruct>,
}

impl RustFile {
    pub fn parse(ast: &syn::File) -> Result<RustFile> {
        let main_export_types = discover_export_types(ast)?;

        let all_structs = find_all_structs(ast)?;

        Ok(RustFile {
            main_export_types,
            all_structs,
        })
    }
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

fn find_all_structs(ast: &syn::File) -> Result<Vec<ElmStruct>> {
    let mut result = vec![];

    for item in &ast.items {
        if let syn::Item::Struct(item_struct) = item {
            let identifier = Identifier(item_struct.ident.to_string());
            match &item_struct.fields {
                syn::Fields::Named(fields) => {
                    result.push(extract_elm_struct(identifier, fields)?);
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
        let ty = match &field.ty {
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
        result.fields.push((ident, ty));
    }
    Ok(result)
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
        let mut file = File::open("src/tests/person.rs").expect("Failed to open file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file");

        let ast = syn::parse_file(&content).expect("Failed to parse file");

        let rust_file = RustFile::parse(&ast)?;
        assert_eq!(1, rust_file.main_export_types.len());
        assert_eq!(
            rust_file.main_export_types[0],
            Identifier("Person".to_string())
        );
        println!("{:#?}", rust_file);
        // panic!();
        Ok(())
    }
}
