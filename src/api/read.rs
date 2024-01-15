pub mod header;

use std::{char, fs, io, usize};

use aho_corasick::{AhoCorasick, PatternID};
use bstr::ByteSlice;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until},
    character::{
        complete::{digit1, line_ending, space0},
        is_space,
    },
    combinator::{map, map_res},
    multi::{count, many0},
    sequence::{pair, preceded, terminated, tuple, Tuple},
    IResult,
};

use crate::api::structure::{self, Subsection, SubsectionEntry};

use super::structure::CrossRefTable;

pub fn read_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let f = fs::read(path)?;

    Ok(f)
}

pub fn read_file(path: &str) -> Result<(), io::Error> {
    let data = read_bytes(path)?;

    let patterns = &[
        "%PDF-",     // Pattern 0 - Start of File
        "\nxref\n",  // Pattern 1 - Start of cross-reference table
        "trailer\n", // Pattern 2 - Start of trailer
        " obj\n",    // Pattern 3 - Start of Object
        "endobj\n",  // Pattern 4 - End of Object
    ];
    let haystack = data.as_slice();

    let ac = AhoCorasick::new(patterns).unwrap();
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }
    println!("{:#?}", matches);
    let header_offset: Vec<_> = matches
        .iter()
        .filter(|&x| x.0.as_usize() == 0)
        .map(|x| x.1)
        .collect();
    let xref_offset: Vec<_> = matches
        .iter()
        .filter(|&x| x.0.as_usize() == 1)
        .map(|x| x.2)
        .collect();
    let trailer_offset: Vec<_> = matches
        .iter()
        .filter(|&x| x.0.as_usize() == 2)
        .map(|x| x.2)
        .collect();
    let objects: Vec<(usize, usize)> = get_objects(matches.as_slice());

    println!("Header Offset: {:?}", header_offset);
    println!("xref_offset: {:?}", xref_offset);
    println!("Trailer Offset: {:?}", trailer_offset);

    header::read_header(&data);
    read_xref(&data, xref_offset[0]);
    read_trailer(&data, trailer_offset[0]);
    read_objects(&data, objects);

    Ok(())
}

pub fn read_objects(data: &[u8], objects: Vec<(usize, usize)>) {
    for kv in objects {
        println!("Start: {:?}; End: {:?}", kv.0, kv.1);
    }
}

pub fn get_objects(matches: &[(PatternID, usize, usize)]) -> Vec<(usize, usize)> {
    let starts: Vec<_> = matches
        .iter()
        .filter(|&x| x.0.as_usize() == 3)
        .map(|x| x.1)
        .collect();
    let ends: Vec<_> = matches
        .iter()
        .filter(|&x| x.0.as_usize() == 4)
        .map(|x| x.1)
        .collect();
    starts.into_iter().zip(ends).collect::<Vec<_>>()
}

pub fn read_xref(data: &[u8], start_offset: usize) -> IResult<&[u8], structure::CrossRefTable> {
    let take_obj_number = map_res(digit1, |d: &[u8]| d.to_str().unwrap().parse::<usize>());
    let take_space = take(1usize);
    let take_num_objects = map_res(digit1, |d: &[u8]| d.to_str().unwrap().parse::<usize>());
    let take_line_ending = line_ending;
    let slice = &data[start_offset..];

    let (input, (obj_number, _, num_objects, _)) = (
        take_obj_number,
        take_space,
        take_num_objects,
        take_line_ending,
    )
        .parse(slice)?;

    let (input, subsection) = read_xref_subsection(input, obj_number, num_objects).unwrap();
    let subsections: Vec<Subsection> = vec![subsection];

    let cross_ref_table = CrossRefTable { subsections };

    Ok((input, cross_ref_table))
}

pub fn read_xref_subsection(
    data: &[u8],
    start_offset: usize,
    num_subsections: usize,
) -> IResult<&[u8], Subsection> {
    let slice = &data[start_offset..];

    let (input, subsections) = read_subsections(slice, num_subsections).unwrap();
    let subsection = Subsection {
        object_number: start_offset,
        num_entries: num_subsections,
        entries: subsections,
    };

    Ok((input, subsection))
}

pub fn read_subsections(
    input: &[u8],
    num_subsections: usize,
) -> IResult<&[u8], Vec<SubsectionEntry>> {
    count(read_subsection, num_subsections)(input)
}

pub fn read_subsection(input: &[u8]) -> IResult<&[u8], SubsectionEntry> {
    map(
        tuple((
            take::<usize, &[u8], _>(10usize),
            take(1usize),
            take(5usize),
            take(1usize),
            take(1usize),
            space0,
            line_ending,
        )),
        |(byte_offset, _, gen_number, _, is_active, _, _)| SubsectionEntry {
            byte_offset: byte_offset.to_str().unwrap().parse::<i32>().unwrap(),
            generation_number: gen_number.to_str().unwrap().parse::<i32>().unwrap(),
            in_use: true,
        },
    )(input)
}

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

#[cfg(test)]
mod tests {
    use std::io;

    use bstr::ByteSlice;
    use nom::AsBytes;

    use super::{read_bytes, read_file, read_xref_subsection};

    #[test]
    fn test_file() -> Result<(), io::Error> {
        read_file("HelloWorld.pdf");
        Ok(())
    }

    #[test]
    fn test_read_bytes() -> Result<(), io::Error> {
        let data = read_bytes("HelloWorld.pdf")?;
        let bytes = data.as_slice();
        println!("Bytes: {:#?}", &bytes[0..10].to_str());
        Ok(())
    }
}
