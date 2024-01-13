use std::{fs, io, ops::Deref, str};

use aho_corasick::AhoCorasick;
use bstr::ByteSlice;
use nom::{
    bytes::complete::{tag, take, take_while1},
    character::{complete::digit1, is_digit},
    combinator::map_res,
    number::complete::be_i32,
    sequence::Tuple,
    IResult, ParseTo,
};

use crate::api::structure;

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
    let take_obj_number = map_res(digit1, |d: &[u8]| d.to_str().unwrap().parse::<i32>());
    let take_space = take(1usize);
    let take_num_objects = map_res(digit1, |d: &[u8]| d.to_str().unwrap().parse::<i32>());
    let slice = &data[start_offset..];

    let (input, (obj_number, _, num_objects)) =
        (take_obj_number, take_space, take_num_objects).parse(slice)?;

    let cross_ref_table = CrossRefTable {
        first_object_object_number: obj_number,
        num_entries: num_objects,
        subsections: None,
    };

    println!("{:?}", cross_ref_table);

    Ok((input, cross_ref_table))
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::{read_bytes, read_file, read_header};

    #[test]
    fn test_file() -> Result<(), io::Error> {
        read_file("HelloWorld.pdf");
        Ok(())
    }
}
