use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::is_space,
    multi::many0,
    sequence::{pair, preceded},
    IResult,
};

pub fn read_trailer(input: &[u8], start_offset: usize) -> IResult<&[u8], &[u8]> {
    let slice = &input[start_offset..];

    let (input, body) = take_trailer_body(slice).unwrap();
    let (input, value) = take_trailer_kv_pair(body).unwrap();
    Ok((input, body))
}

pub fn take_trailer_body(input: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(
        tag::<&str, &[u8], nom::error::Error<&[u8]>>("<<"), // Capture start of body of trailer
        take_until("startxref\n"),
    )(input)
}

pub fn take_trailer_kv_pair(input: &[u8]) -> IResult<&[u8], Vec<(&[u8], &[u8])>> {
    many0(preceded(
        tag("/"),
        pair(take_trailer_kv_key, take_trailer_kv_value),
    ))(input)
}

pub fn take_trailer_kv_key(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(is_space)(input)
}

pub fn take_trailer_kv_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until("/")(input)
}
