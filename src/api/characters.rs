/// Characters in PDFs are represented as a table containing the following
/// information:
///  - Decimal Representation
///  - Hexadeicaml Representation
///  - Octal Representation
///  - Name
///
///  In addition, Characters in the PDF standard are divided into three classes
///  (represented in the Character struct by the CharacterClass enum):
///   - Regular
///   - Delimited
///   - White-Space
///
trait Character {
    fn name(&self) -> &'static str;
    fn decimal(&self) -> i32;
    fn character_class(&self) -> CharacterClass;
}

/// "The PDF character set is divided into three classes referred to as
/// regular, delimiter, and white-space. This classification enables the
/// grouping of characters into tokens including separating syntactic constructs
/// such as names and numbers from each other."
/// See ISO 32000-2:2020 pp. 21-22
#[derive(Debug, PartialEq, Eq)]
enum CharacterClass {
    Regular,
    Delimiter,
    WhiteSpace,
}

/// White Space Characters in the PDF standard as defined in sec. 7.2, Table 1
/// (pg. 22) of the PDF standard.
#[derive(Debug)]
enum WhiteSpaceCharacter {
    Null,
    HorizontalTab,
    LineFeed,
    FormFeed,
    CarriageReturn,
    Space,
}

impl Character for WhiteSpaceCharacter {
    fn name(&self) -> &'static str {
        match self {
            WhiteSpaceCharacter::Null => "NUL",
            WhiteSpaceCharacter::HorizontalTab => "HT",
            WhiteSpaceCharacter::LineFeed => "LF",
            WhiteSpaceCharacter::FormFeed => "FF",
            WhiteSpaceCharacter::CarriageReturn => "CR",
            WhiteSpaceCharacter::Space => "SP",
        }
    }

    fn decimal(&self) -> i32 {
        match self {
            WhiteSpaceCharacter::Null => 0,
            WhiteSpaceCharacter::HorizontalTab => 9,
            WhiteSpaceCharacter::LineFeed => 10,
            WhiteSpaceCharacter::FormFeed => 12,
            WhiteSpaceCharacter::CarriageReturn => 13,
            WhiteSpaceCharacter::Space => 32,
        }
    }

    fn character_class(&self) -> CharacterClass {
        CharacterClass::WhiteSpace
    }
}

#[cfg(test)]
mod tests {
    use super::WhiteSpaceCharacter;
    use crate::api::characters::{Character, CharacterClass};

    #[test]
    fn test_white_space_char_class() {
        let null = WhiteSpaceCharacter::Null;
        assert_eq!(null.character_class(), CharacterClass::WhiteSpace)
    }
}
