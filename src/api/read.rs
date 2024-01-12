use std::{
    fmt::Write,
    fs::{self, File},
    io::{self, Read},
    str,
};

use bstr::ByteSlice;
use nom::{
    bytes::complete::{tag, take, take_while1},
    character::is_alphabetic,
    sequence::Tuple,
    AsBytes, IResult,
};

use super::structure::Header;

pub fn read_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let f = fs::read(path)?;

    Ok(f)
}

pub fn read_header(data: &Vec<u8>) -> IResult<&str, Header> {
    // The defined pattern to match against use bstr
    let pattern = "%PDF-";
    // The bytes offset defined in the PDF standrd from the initial header byte
    // to End
    let offset = 15; // Needs to be 16, but for testing it's 15 due to non-UTF8 error
    let mut matches = vec![];
    for tag in data.find_iter(pattern) {
        matches.push(tag);
    }
    let index = matches[0];
    let slice = &data[index..=offset];
    let slice = str::from_utf8(slice).unwrap();
    println!("{slice:?}");

    let clear_pdf_tag = tag("%PDF-");
    let take3 = take(3usize);
    let (input, (_, version)) = (clear_pdf_tag, take3).parse(slice)?;
    let header = Header {
        version: version.to_string(),
        comment: None,
    };

    Ok((input, header))
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::{read_bytes, read_header};

    #[test]
    fn test_header() -> Result<(), io::Error> {
        let data = read_bytes("HelloWorld.pdf")?;
        let (input, major) = read_header(&data).unwrap();
        println!("Input: {input:?}\nMajor: {major:?}");
        Ok(())
    }
}
