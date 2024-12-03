pub fn usize_from_enum_strln(n: usize, line: &str, sep: &str, length: usize) -> Vec<usize> {
    let result: Vec<usize> = line
        .split(sep)
        .map(|i| {
            i.parse::<usize>().expect(
                format!(
                    "Could not parse {} from input line {}: {} into usize!",
                    i,
                    n,
                    line,
                ).as_str()
            )
        })
        .collect();

    if result.len() != length {
        panic! {
            "Length error! Input line {}: {}, expected length: {}, found: {}",
            n,
            line,
            length,
            result.len(),
        }
    }

    result
}

pub fn usize_from_unbounded_enum_strln(n: usize, line: &str, sep: &str) -> Vec<usize> {
    let result: Vec<usize> = line
        .split(sep)
        .map(|i| {
            i.parse::<usize>().expect(
                format!(
                    "Could not parse {} from input line {}: {} into usize!",
                    i,
                    n,
                    line,
                ).as_str()
            )
        })
        .collect();

    result
}