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
