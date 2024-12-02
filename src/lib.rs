use std::fs::read_to_string;

pub fn read_input(day: i32) -> String {
    let path = format!("inputs/day{}.txt", day);
    let content = read_to_string(path).unwrap();
    return content;
}

pub fn parse_row_major<T: std::str::FromStr + std::default::Default>(input: &str) -> Vec<Vec<T>> {
    let lines = input.split("\n");
    let mut rows: Vec<Vec<T>> = Vec::new();
    for line in lines {
        rows.push(
            line.split(" ")
                .filter(|&x| !x.is_empty())
                .map(
                    |x| x.parse::<T>().unwrap_or_default()
                )
                .collect()
        );
    }
    return rows;
}
