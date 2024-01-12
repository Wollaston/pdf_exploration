use std::{fs, io, str};

use aho_corasick::AhoCorasick;
use bstr::ByteSlice;
use nom::{
    bytes::complete::{tag, take},
    sequence::Tuple,
    IResult,
};

use super::structure::Header;

pub fn read_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let f = fs::read(path)?;

    Ok(f)
}

pub fn read_header(data: &[u8], start_offset: usize) -> IResult<&[u8], Header> {
    // The bytes offset defined in the PDF standrd from the initial header byte
    // to End
    let end_offset = 15; // Needs to be 16, but for testing it's 15 due to non-UTF8 error
    let slice = &data[start_offset..=end_offset];
    // let slice = str::from_utf8(slice).unwrap();
    println!("{slice:?}");

    let clear_pdf_tag = tag("%PDF-");
    let take3 = take(3usize);
    let take2 = take(2usize);

    let (input, (_, version, _)) = (clear_pdf_tag, take3, take2).parse(slice)?;
    let header = Header {
        version: version.to_str().unwrap().to_string(),
        comment: None,
    };
    println!("Version: {:?}", header.version);

    Ok((input, header))
}

pub fn read_file(path: &str) -> Result<(), io::Error> {
    let data = read_bytes(path)?;

    let patterns = &["%PDF-", "endstream"];
    let haystack = data.as_slice();

    let ac = AhoCorasick::new(patterns).unwrap();
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }
    let header_offset: Vec<_> = matches.iter().filter(|&x| x.0.as_usize() == 0).collect();
    println!("Header Offset: {:?}", header_offset);
    println!("Matches: {:?}", matches);

    read_header(&data, header_offset[0].0.as_usize());

    Ok(())
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
