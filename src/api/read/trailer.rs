use bstr::ByteSlice;
use nom::bytes::complete::{is_not, take_while};
use nom::character::complete::{multispace0, space0, space1};
use nom::character::{is_alphabetic, is_digit};
use nom::combinator::{map_res, rest};
use nom::error::ErrorKind;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::is_space,
    multi::many0,
    sequence::{pair, preceded},
    IResult,
};

use crate::api::structure::{Object, ObjectStatus, TrailerKey, TrailerValue};

/// The PDF trailer consists of the following structure:
/// - The start is denoted by the "trailer" keyword
/// - The trailer entries are delimited by "<< . . . >>"
/// - Each entry consists of a key value pair
/// - Each trailer is ended by three lines, consisting of:
///     - The "startxref" keyword
///     - The byte offset of the last cross-reference section (as an integer)
///     - The PDF end-of-file keyword "%%EOF"
pub fn read_trailer(input: &[u8], start_offset: usize) -> IResult<&[u8], &[u8]> {
    let slice = &input[start_offset..];

    let (_, body) = take_trailer_body(slice).unwrap();
    let (input, _) = take_trailer_kv_pair(body).unwrap();
    Ok((input, body))
}

/// The body of a PDF trailer consists of a series of key value pairs, together
/// delimieted by "<< . . . >>"
pub fn take_trailer_body(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, _) = take_until("<<")(input)?;
    preceded(
        tag::<&str, &[u8], nom::error::Error<&[u8]>>("<<"), // Capture start of body of trailer
        take_until(">>"),
    )(input)
}

pub fn take_trailer_kv_pairs(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    let (input, pairs) = many0(preceded(tag("/"), take_until("/")))(input)?;
    Ok((input, pairs))
}

/// Each trailer key value pair consists of a Key, as defined in the PDF standard (see
/// sec 7.5.5, Table 15, and Annex E), and a corresponding value with an associated
/// type, as defined in the aforementioned tables.
pub fn take_trailer_kv_pair(input: &[u8]) -> IResult<&[u8], Vec<(TrailerKey, TrailerValue)>> {
    let (input, pairs) = many0(preceded(
        tag("/"),
        pair(take_trailer_kv_key, take_trailer_kv_value),
    ))(input)?;
    let pairs = pairs
        .into_iter()
        .filter(|pair| pair.0 != TrailerKey::Unknown)
        .collect::<Vec<_>>();
    let mut parsed_pairs: Vec<(TrailerKey, TrailerValue)> = vec![];
    for pair in pairs {
        match pair.0 {
            TrailerKey::Size => parsed_pairs.push((pair.0, parse_integer(pair.1).unwrap().1)),
            TrailerKey::Root | TrailerKey::Info => {
                parsed_pairs.push((pair.0, parse_object(pair.1).unwrap().1))
            }
            TrailerKey::ID => parsed_pairs.push((pair.0, parse_array(pair.1).unwrap().1)),
            _ => parsed_pairs.push((pair.0, parse_unknown(pair.1).unwrap().1)),
        }
    }
    Ok((input, parsed_pairs))
}

/// The PDF trailer keys are defined in the PDF standard. See sec 7.5.5, Table 15,
/// and Annex E.
pub fn take_trailer_kv_key(input: &[u8]) -> IResult<&[u8], TrailerKey> {
    map_res(take_till(is_space), |key: &[u8]| {
        Ok::<TrailerKey, ErrorKind>(match key.to_str().unwrap() {
            "Size" => TrailerKey::Size,
            "Pref" => TrailerKey::Prev,
            "Root" => TrailerKey::Root,
            "Encrypt" => TrailerKey::Encrypt,
            "Info" => TrailerKey::Info,
            "ID" => TrailerKey::ID,
            _ => TrailerKey::Unknown,
        })
    })(input)
}

/// Each PDF trailer value has an associated type, as defined in the PDF standard.
/// See sec 7.5.5, Table 15, and Annex E.
pub fn take_trailer_kv_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, value) = take_until("/")(input)?;
    let (_, value) = preceded(space0, rest)(value)?;
    Ok((input, value))
}

pub fn parse_integer(input: &[u8]) -> IResult<&[u8], TrailerValue> {
    println!("{:?}", input.as_bstr());
    let integer = input.to_str().unwrap().parse::<i32>().unwrap();
    Ok((input, TrailerValue::Integer(integer)))
}

pub fn parse_object(input: &[u8]) -> IResult<&[u8], TrailerValue> {
    let (input, (object_number, _, gen_number, _, status)) = tuple((
        take_while(is_digit),
        space1,
        take_while(is_digit),
        space1,
        take_while(is_alphabetic),
    ))(input)?;
    let object_number = object_number.to_str().unwrap().parse::<i32>().unwrap();
    let generation_number = gen_number.to_str().unwrap().parse::<i32>().unwrap();
    let status = match status.to_str().unwrap() {
        "obj" => ObjectStatus::Definition,
        "R" => ObjectStatus::Reference,
        _ => ObjectStatus::Unknown,
    };
    Ok((
        input,
        TrailerValue::Object(Object {
            object_number,
            generation_number,
            status,
        }),
    ))
}

pub fn parse_array(input: &[u8]) -> IResult<&[u8], TrailerValue> {
    let (_, output) = delimited(tag("["), is_not("]"), tag("]"))(input)?;
    let (_, output) = delimited(multispace0, is_not(" \t"), multispace0)(output)?;
    let (_, output) = separated_pair(is_not("\n"), tag("\n"), is_not("\n"))(output)?;
    let (_, output1) = delimited(tag("<"), is_not(">"), tag(">"))(output.0)?;
    let (input, output2) = delimited(tag("<"), is_not(">"), tag(">"))(output.1)?;
    let output = vec![
        String::from_utf8(output1.to_vec()).unwrap(),
        String::from_utf8(output2.to_vec()).unwrap(),
    ];
    Ok((input, TrailerValue::Array(output)))
}

pub fn parse_unknown(input: &[u8]) -> IResult<&[u8], TrailerValue> {
    Ok((input, TrailerValue::Unknown))
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::*;

    #[test]
    fn test_trailer_body() {
        let input = "trailer
<</Size 14/Root 12 0 R
/Info 13 0 R
/ID [ <2379A69CC127F883136B24B3AABCB40D>
<2379A69CC127F883136B24B3AABCB40D> ]
/DocChecksum /32059j2fin
>>
startxref";

        let bytes = input.as_bytes();
        let (input, body) = take_trailer_body(bytes).unwrap();
        println!(
            "Input: {:#?}\nBody: {:#?}",
            input.to_str().unwrap(),
            body.to_str().unwrap()
        );
    }

    #[test]
    fn test_take_trailer_kv_pair() {
        let input = "/Size 14/Root 12 0 R
/Info 13 0 R
/ID [ <2379A69CC127F883136B24B3AABCB40D>
<2379A69CC127F883136B24B3AABCB40D> ]
/DocChecksum /34352532";

        let bytes = input.as_bytes();
        let (input, pairs) = take_trailer_kv_pair(bytes).unwrap();
        println!("Input: {:#?}", input.to_str().unwrap());
        for pair in pairs {
            println!("Pair: {:#?} | {:#?}", pair.0, pair.1);
        }
    }
}
