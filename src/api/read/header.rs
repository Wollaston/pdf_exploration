use bstr::ByteSlice;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, not_line_ending},
    sequence::tuple,
    IResult,
};

use crate::api::structure;

pub fn read_header(input: &[u8]) -> IResult<&[u8], structure::Header> {
    let (input, first_row) = take_first_row(input)?;
    let (input, second_row) = take_second_row(input)?;

    let (version, _) = tag("%PDF-")(first_row)?;

    let header = structure::Header {
        version: String::from_utf8(version.to_vec()).unwrap(),
        comment: second_row.is_utf8(),
    };

    Ok((input, header))
}

pub fn take_first_row(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, (_, content, _)) = tuple((multispace0, not_line_ending, multispace0))(input)?;
    Ok((input, content))
}

pub fn take_second_row(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, (_, content, _)) = tuple((multispace0, not_line_ending, multispace0))(input)?;
    Ok((input, content))
}

// pub fn take_second_row(input: &[u8]) -> IResult<&[u8], &[u8]> {}

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
