/// "The PDF file begins with the 5 characters “%PDF–” and byte offsets shall
/// be calculated from the␍PERCENT SIGN (25h)." This is followed by the version
/// of the PDF. For example, "%PDF–1.6".
///
/// "If a PDF file contains binary data, as most do (see 7.2, "Lexical conventions"),
/// the header line shall be␍immediately followed by a comment line containing
/// at least four binary characters–that is, characters␍whose codes are 128 or greater."
#[derive(Debug)]
pub struct Header {
    pub text: String,
    pub version: String,
    pub comment: String,
}

struct Body {}

struct XRefTable {}

struct Trailer {}
