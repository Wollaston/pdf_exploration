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
struct Character {
    name: String,
    character_class: CharacterClass,
    decimal: i32,
    glyph: Option<char>,
}

/// "The PDF character set is divided into three classes referred to as
/// regular, delimiter, and white-space. This classification enables the
/// grouping of characters into tokens including separating syntactic constructs
/// such as names and numbers from each other."
/// See ISO 32000-2:2020 pp. 21-22
enum CharacterClass {
    Regular,
    Delimiter,
    WhiteSpae,
}
