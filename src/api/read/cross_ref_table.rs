use bstr::ByteSlice;
use nom::{
    bytes::complete::take,
    character::complete::{digit1, line_ending, space0},
    combinator::{map, map_res},
    multi::count,
    sequence::{tuple, Tuple},
    IResult,
};

use crate::api::structure::{self, CrossRefTable, Subsection, SubsectionEntry};

/// The cross-reference table begins with the keyword "xref" and contains one or more
/// cross-reference sections.
///
/// Each cross-reference section begins with two integers representing a contiguous
/// range of objects. The integers are seperated by a space and together terminated by
/// and EOL character. The first integer represents the object number of the first
/// object in the subsection, and the second the numer of entries in the subsection.
/// For example, "10 2" denotes a subsection starting with object number 10 and spanning
/// two objects; accordingly, this subsection consists of objets 10 and 11.
///
/// Each section consists of the noted number of subsections, in the format of:
/// nnnnnnnnnn ggggg n eol
///
/// Where:
/// - nn..nn is the 10-digit byte offset of the object
/// - ggggg is the 5-digit generation number
/// - n is a keyword identifying if the entry is in use or not
///     - n denotes in-use; f denotes a free entry
/// - a 2-digit EOL sequence
pub fn read_cross_ref_table(
    data: &[u8],
    start_offset: usize,
) -> IResult<&[u8], structure::CrossRefTable> {
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

    let (input, subsection) = read_cross_ref_section(input, obj_number, num_objects).unwrap();
    let subsections: Vec<Subsection> = vec![subsection];

    let cross_ref_table = CrossRefTable { subsections };

    Ok((input, cross_ref_table))
}

/// Each cross-reference table section begins with two integers, separated by a
/// space and terminated by an eol character, noting the object number of the first
/// object and the number of entries in the section.
pub fn read_cross_ref_section(
    data: &[u8],
    start_offset: usize,
    num_entries: usize,
) -> IResult<&[u8], Subsection> {
    let slice = &data[start_offset..];

    let (input, subsections) = read_subsections(slice, num_entries).unwrap();
    let subsection = Subsection {
        object_number: start_offset,
        num_entries,
        entries: subsections,
    };

    Ok((input, subsection))
}

/// Each cross-reference table section contains one more more subsections.
/// The number of subsections is denoted in the section header as the second
/// integer.
pub fn read_subsections(
    input: &[u8],
    num_subsections: usize,
) -> IResult<&[u8], Vec<SubsectionEntry>> {
    count(read_subsection, num_subsections)(input)
}

/// Each cross-reference table sub-section consists of three main parts in the format of:
/// nnnnnnnnnn ggggg n eol
///
/// Where:
/// - nn..nn is the 10-digit byte offset of the object
/// - ggggg is the 5-digit generation number
/// - n is a keyword identifying if the entry is in use or not
///     - n denotes in-use; f denotes a free entry
/// - a 2-digit EOL sequence
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
