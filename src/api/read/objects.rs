use aho_corasick::PatternID;

pub fn read_objects(data: &[u8], objects: Vec<(usize, usize)>) {
    for kv in objects {
        println!("Start: {:?}; End: {:?}", kv.0, kv.1);
    }
}

pub fn get_objects(matches: &[(PatternID, usize, usize)]) -> Vec<(usize, usize)> {
    let starts: Vec<_> = matches
        .iter()
        .filter(|&x| x.0.as_usize() == 3)
        .map(|x| x.1)
        .collect();
    let ends: Vec<_> = matches
        .iter()
        .filter(|&x| x.0.as_usize() == 4)
        .map(|x| x.1)
        .collect();
    starts.into_iter().zip(ends).collect::<Vec<_>>()
}
