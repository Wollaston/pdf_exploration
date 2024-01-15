pub mod cross_ref_table;
pub mod header;
pub mod objects;
pub mod trailer;

use aho_corasick::AhoCorasick;
use std::{fs, io};

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
    let objects: Vec<(usize, usize)> = objects::get_objects(matches.as_slice());

    println!("Header Offset: {:?}", header_offset);
    println!("xref_offset: {:?}", xref_offset);
    println!("Trailer Offset: {:?}", trailer_offset);

    header::read_header(&data);
    cross_ref_table::read_xref(&data, xref_offset[0]);
    trailer::read_trailer(&data, trailer_offset[0]);
    objects::read_objects(&data, objects);

    Ok(())
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::*;

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
