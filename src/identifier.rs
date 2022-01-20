/// Identifier as it appears in Rust. This means the type names are in (upper)
/// camel case and the variable names are in snake case.
///
/// I noted during the refactoring that maybe it could be good to split this
/// TypeIdentifier, MemberIdentifier and maybe VariantConstructor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

#[derive(PartialEq, Eq)]
pub enum LetterCase {
    UpperCase,
    LowerCase,
}

impl Identifier {
    pub fn new(name: impl ToString) -> Identifier {
        Identifier(name.to_string())
    }

    /// Assumes that the string contained inside the identifier is snake case
    /// and converts it to camel case.
    /// This is required for elm struct fields.
    pub fn camel_case(&self, case: LetterCase) -> String {
        let mut result = String::new();
        let mut next_uppercase = case == LetterCase::UpperCase;
        for c in self.0.chars() {
            if c == '_' {
                next_uppercase = true;
            } else if next_uppercase {
                result.push(c.to_ascii_uppercase());
                next_uppercase = false;
            } else {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LetterCase::*;

    #[test]
    fn test_camel_case() {
        let identifier = Identifier("snake_case".to_string());
        assert_eq!(identifier.camel_case(UpperCase), "SnakeCase");
        assert_eq!(identifier.camel_case(LowerCase), "snakeCase");
    }
}
