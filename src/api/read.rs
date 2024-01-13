use std::{fs, io};

use aho_corasick::AhoCorasick;
use bstr::ByteSlice;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, line_ending, space0},
    combinator::{map, map_res},
    multi::count,
    sequence::{tuple, Tuple},
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

    let patterns = &["%PDF-", "\nxref\n"];
    let haystack = data.as_slice();

    let ac = AhoCorasick::new(patterns).unwrap();
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }
    let header_offset: Vec<_> = matches.iter().filter(|&x| x.0.as_usize() == 0).collect();
    let xref_offset: Vec<_> = matches.iter().filter(|&x| x.0.as_usize() == 1).collect();

    println!("Header Offset: {:?}", header_offset);
    println!("xref_offset: {:?}", xref_offset);
    println!("Matches: {:?}", matches);

    read_header(&data, header_offset[0].0.as_usize());
    read_xref(&data, xref_offset[0].2);

    Ok(())
}

pub fn read_header(data: &[u8], start_offset: usize) -> IResult<&[u8], structure::Header> {
    // The bytes offset defined in the PDF standrd from the initial header byte
    // to End
    let end_offset = 16; // Needs to be 16, but for testing it's 15 due to non-UTF8 error
    let slice = &data[start_offset..=end_offset];
    // let slice = str::from_utf8(slice).unwrap();
    println!("{slice:?}");

    let clear_pdf_tag = tag("%PDF-");
    let take3 = take(3usize);
    let take1 = take(1usize);
    let percent = take(1usize);

    let (input, (_, version, _, percent)) = (clear_pdf_tag, take3, take1, percent).parse(slice)?;
    let header = structure::Header {
        version: String::from_utf8(version.to_vec()).unwrap(),
        comment: percent.to_str().unwrap() == "%",
    };
    println!("Version: {:?}", header.version);
    println!("Remainder: {:?}", input.as_bstr());

    Ok((input, header))
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

    println!("{:#?}", cross_ref_table);

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

#[cfg(test)]
mod tests {
    use std::io;

    use super::read_file;

    #[test]
    fn test_file() -> Result<(), io::Error> {
        read_file("HelloWorld.pdf");
        Ok(())
    }
}
