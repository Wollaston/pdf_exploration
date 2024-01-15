use bstr::ByteSlice;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, not_line_ending},
    sequence::tuple,
    IResult,
};

use crate::api::structure;

/// A PDF file contains a one or two line header (usually two lines). The first line
/// denotes the version of the PDF standard used and includes the PDF file start
/// keyword "%PDF-", which is followed by the version and an EOL character. The
/// optional, but common, second line includes a "%" followed by at least four
/// characters whose codes are greater than 128, denoting that the file contains
/// binary characters. The absence of this "comment" line indicates the file consists
/// of only ASCII characters with a maximum code of 128, and it also indicates that
/// the file can be read as plain text.
pub fn read_header(input: &[u8]) -> IResult<&[u8], structure::Header> {
    let (input, first_row) = take_first_row(input)?;
    let (input, second_row) = take_second_row(input)?;

    let (version, _) = tag("%PDF-")(first_row)?;

    let header = structure::Header {
        version: String::from_utf8(version.to_vec()).unwrap(),
        comment: second_row.is_ascii(),
    };

    Ok((input, header))
}

/// Reads the first row of the header, which starts with the start PDF keyword "%PDF-"
/// and may or may not be the first bytes in the file.
pub fn take_first_row(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, (_, content, _)) = tuple((multispace0, not_line_ending, multispace0))(input)?;
    Ok((input, content))
}

/// Reads the second row of the header and determines if it contains non-ASCII characters.
/// If it does, it flags the comment as "true"; otherwise, if all are ASCII characters,
/// it is flagged false to indicate that the file does not contain binary characters.
pub fn take_second_row(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, (_, content, _)) = tuple((multispace0, not_line_ending, multispace0))(input)?;
    Ok((input, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_header() {
        let input = "%PDF-1.6\n%äüöß";
        let bytes = input.as_bytes();
        println!("{:#?}", bytes.as_bstr());
        let (input, header) = read_header(bytes).unwrap();
        println!("{:#?}", input);
        println!("{:#?}", header);
    }

    #[test]
    fn test_first_row() {
        let input = "%PDF-1.6\nSECOND_ROW";
        let bytes = input.as_bytes();
        let (input, first_row) = take_first_row(bytes).unwrap();
        println!("Input: {:#?}", input.to_str());
        println!("First Row: {:#?}", first_row.to_str());

        let input = "%PDF-1.6\r\n%SECOND_ROW";
        let bytes = input.as_bytes();
        let (input, first_row) = take_first_row(bytes).unwrap();
        println!("Input: {:#?}", input.to_str());
        println!("First Row: {:#?}", first_row.to_str());
    }

    #[test]
    fn test_second_row() {
        let input = "\n%PDF-1.6\nSECOND_ROW";
        let bytes = input.as_bytes();
        let (input, first_row) = take_second_row(bytes).unwrap();
        println!("Input: {:#?}", input.to_str());
        println!("Match: {:#?}", first_row.to_str());

        let input = "\r\n%PDF-1.6\r\nSECOND_ROW";
        let bytes = input.as_bytes();
        let (input, first_row) = take_second_row(bytes).unwrap();
        println!("Input: {:#?}", input.to_str());
        println!("Match: {:#?}", first_row.to_str());

        let input = "%PDF-1.6\nSECOND_ROW";
        let bytes = input.as_bytes();
        let (input, first_row) = take_second_row(bytes).unwrap();
        println!("Input: {:#?}", input.to_str());
        println!("Match: {:#?}", first_row.to_str());
    }

    #[test]
    fn test_first_two_rows() {
        let input = "\n%PDF-1.6\nSECOND_ROW";
        println!("Input: {:#?}", input);
        let bytes = input.as_bytes();
        let (input, first_row) = take_first_row(bytes).unwrap();
        let (input, second_row) = take_second_row(input).unwrap();
        println!("First Row: {:#?}", first_row.to_str());
        println!("Second Row: {:#?}", second_row.to_str());
        println!("Remainder: {:#?}", input.to_str());

        let input = "\r\n%PDF-1.6\nSECOND_ROW\nTHIRD ROW";
        println!("Input: {:#?}", input);
        let bytes = input.as_bytes();
        let (input, first_row) = take_first_row(bytes).unwrap();
        let (input, second_row) = take_second_row(input).unwrap();
        println!("First Row: {:#?}", first_row.to_str());
        println!("Second Row: {:#?}", second_row.to_str());
        println!("Remainder: {:#?}", input.to_str());

        let input = "EXTRA\n%PDF-1.6\r\nSECOND_ROW";
        println!("Input: {:#?}", input);
        let bytes = input.as_bytes();
        let (input, first_row) = take_first_row(bytes).unwrap();
        let (input, second_row) = take_second_row(input).unwrap();
        println!("First Row: {:#?}", first_row.to_str());
        println!("Second Row: {:#?}", second_row.to_str());
        println!("Remainder: {:#?}", input.to_str());
    }
}
