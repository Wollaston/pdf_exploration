use std::{
    fs::File,
    io::{self, Read},
    str,
};

use crate::api::structure;

/// Opens a local PDF file and reads its content into associated
/// associated structs:
///  - Header
///  - Body
///  - XRef Table
///  - Trailer
#[derive(Debug, Clone, Copy)]
struct PDFContent {
    path: &'static str,
}

impl PDFContent {
    fn new(path: &'static str) -> Self {
        Self { path }
    }

    fn read_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut f = File::open(self.path)?;
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, BufRead, BufReader},
    };

    use crate::api::structure;

    use super::PDFContent;

    // #[test]
    // fn read_bytes_from_local_file() -> Result<(), io::Error> {
    //     let buffer = PDFContent::new("HelloWorld.pdf").read_bytes()?;
    //     for byte in buffer {
    //         println!("Byte {} as ASCII {}", byte, byte as char)
    //     }
    //     Ok(())
    // }

    #[test]
    fn read_header_from_local_file() -> Result<(), io::Error> {
        let f = File::open("HelloWorld.pdf")?;
        let mut reader = BufReader::new(f);
        let mut text = String::new();
        let mut comment = String::new();

        reader.read_line(&mut text)?;
        reader.read_line(&mut comment)?;
        let version = &mut text.split_at(text.find('-').unwrap() + 1);
        let header = structure::Header {
            text: text.trim().to_string(),
            version: format!("PDF Version {}", version.1.trim()),
            comment: comment.trim().to_string(),
        };

        println!("{:#?}", header);

        Ok(())
    }
}
