///
/// The Basic PDF struct, containing the follow core elements:
///
/// "A basic conforming PDF file shall be constructed of the following four elements:
/// - A one-line header identifying the version of the PDF specification to which the
///     PDF file conforms
/// - A body containing the objects that make up the document contained in the PDF file
/// - A cross-reference table containing information about the indirect objects in the
///     PDF file
/// - A trailer giving the location of the cross-reference table and of certain special
///     objects within the␍body of the PDF file."
///     See sec. 7.5.1 of the ISO 3200-2:2020 spec (pg. 53)
///
/// ______________
/// | __________  |
/// | | Header  | |
/// | |_________| |
/// | |  Body   | |
/// | |         | |
/// | |         | |
/// | |         | |
/// | |_________| |
/// | |  X-Ref  | |
/// | |  table  | |
/// | |_________| |
/// | | Trailer | |
/// | |_________| |
/// |_____________|
///
/// An implementation of figure 2, pg. 54, of the spec.
///
#[derive(Debug)]
pub struct PDF {
    header: Header,
    body: Body,
    cross_ref_table: CrossRefTable,
    trailer: Trailer,
}

///
/// "The PDF file begins with the 5 characters “%PDF–” and byte offsets shall
/// be calculated from the␍PERCENT SIGN (25h)." This is followed by the version
/// of the PDF in {Major}.{Minor} version format. For example, "%PDF–1.6".
///
/// "If a PDF file contains binary data, as most do (see 7.2, "Lexical conventions"),
/// the header line shall be␍immediately followed by a comment line containing
/// at least four binary characters–that is, characters␍whose codes are 128 or greater."
///
/// If the file does not contain this comment, it can be treated as text data.
///
/// Accordingly, the Header struct includes the version and an optional comment
/// field, in the event that the PDF contains no binary data.
///
#[derive(Debug)]
pub struct Header {
    pub version: String,
    pub comment: Option<String>,
}

///
/// "The body of a PDF file shall consist of a sequence of indirect objects representing
/// the contents of a document. The objects, which are of the basic types described in 7.3,
/// "Objects" represent components of the document such as fonts, pages, and sampled images.
/// Beginning with PDF 1.5, the body can also contain object streams, each of which contains
/// a sequence of indirect objects; see 7.5.7, "Object streams"."
///
/// Accordingly, the Body struct contains a Vec of Objects.
///
#[derive(Debug)]
struct Body {
    objects: Vec<Object>,
}

///
/// The cross-reference table permits random access to the indirect objects of the PDF
/// file by specifying the byte offset of it's start. the table begins with a line
/// containing the keyword "xref" and subsections following.
///
#[derive(Debug)]
struct CrossRefTable {
    subsections: Vec<Subsection>,
}

///
/// The trailer enables a PDF processor to quickly find key elements of the file:
///     - The last line in the file is marked by the EOF marker: "%%EOF"
///     - The third-to-last line is the keyword "startxref" which is followed
///         on the second-to-last line by the byte-offset of the start of the
///         cross-reference table so that it can be quickly accessed by reading
///         the PDF from the end first.
///
/// The trailer itself starts with the "trailer" keyword and contains a series
/// of key-value pairs that are together enclosed by double angle brackets:
///     << . . . >>
///
/// For a list of key-value pair options available in the trailer, including
/// where they are required or optional, see Table 15, sec. 7.5.5 (pg. 58)
/// of the PDF ISO 32000-2:2020 spec.
///
#[derive(Debug)]
struct Trailer {
    entries: Vec<(TrailerKey, String)>,
}

#[derive(Debug)]
pub enum TrailerKey {
    Size,
    Prev,
    Root,
    Encrypt,
    Info,
    ID,
}

///
/// A PDF object is constructed of nine basic object types. These objects can
/// be labeled with an "Object Number" and a "Generation Number" so that they
/// can be refered to by other objects as an "Indirect Object."
///
#[derive(Debug)]
struct Object {
    object_number: i32,
    generation_number: i32,
}

///
/// An enum representing the 9 types of objects defined in the PDF standard.
/// See sec 7.3 (pg. 24)
///
pub enum ObjectTypes {
    Boolean,
    Integer,
    RealNumber,
    String,
    Name,
    Array,
    Dictionary,
    Stream,
    Null,
}

///
/// Each cross-reference table subsection begins with two integers seperated
/// by a space and terminated by an EOL marker:
///     - The object number of the first object in this subsection
///     - The number of entries in the subsection
///
/// It is then followed by the subsection entries.
///
#[derive(Debug)]
struct Subsection {
    object_number: i32,
    num_entries: i32,
    entries: Vec<SubsectionEntry>,
}

///
/// Each subsection of the cross-reference table is 20 bytes long with the
/// following elements:
///     - 10-digit byte offset (padded with leading 0's)
///     - A space
///     - 5-digit generation number
///     - A space
///     - A keyword identifying if the entry is in-use
///         - 'n' designates an in-use entry; 'f' a free entry
///     - A 2-character end-of-line sequence, which can be one of the following:
///         - 'SP CR'
///         - 'SP LF'
///         - 'CR LF'
///
/// Accordingly, each subsection entry is exactly 20-bytes.
///
#[derive(Debug)]
struct SubsectionEntry {
    byte_offset: i32,
    generation_number: i32,
    in_use: bool,
}
