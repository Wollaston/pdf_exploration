use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};

pub fn is_pdf_eol(chr: u8) -> bool {
    chr == b'\n' || chr == b'\r'
}
