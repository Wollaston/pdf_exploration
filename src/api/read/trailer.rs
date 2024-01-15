use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::is_space,
    multi::many0,
    sequence::{pair, preceded},
    IResult,
};

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

    let (input, body) = take_trailer_body(slice).unwrap();
    let (input, value) = take_trailer_kv_pair(body).unwrap();
    Ok((input, body))
}

/// The body of a PDF trailer consists of a series of key value pairs, together
/// delimieted by "<< . . . >>"
pub fn take_trailer_body(input: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(
        tag::<&str, &[u8], nom::error::Error<&[u8]>>("<<"), // Capture start of body of trailer
        take_until("startxref\n"),
    )(input)
}

/// Each trailer key value pair consists of a Key, as defined in the PDF standard (see
/// sec 7.5.5, Table 15, and Annex E), and a corresponding value with an associated
/// type, as defined in the aforementioned tables.
pub fn take_trailer_kv_pair(input: &[u8]) -> IResult<&[u8], Vec<(&[u8], &[u8])>> {
    many0(preceded(
        tag("/"),
        pair(take_trailer_kv_key, take_trailer_kv_value),
    ))(input)
}

/// The PDF trailer keys are defined in the PDF standard. See sec 7.5.5, Table 15,
/// and Annex E.
pub fn take_trailer_kv_key(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(is_space)(input)
}

/// Each PDF trailer value has an associated type, as defined in the PDF standard.
/// See sec 7.5.5, Table 15, and Annex E.
pub fn take_trailer_kv_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until("/")(input)
}
