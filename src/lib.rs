use std::fs::read_to_string;

pub fn read_input(day: i32) -> String {
    let path = format!("inputs/day{}.txt", day);
    let content = read_to_string(path).unwrap();
    return content;
    // let path_string = &format!("inputs/day{}.txt", day);
    // let path = Path::new(path_string);
    // let display = path.display();   
    
    // // open the path in read-only mode. Panic if not found.
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };
    
    // let mut s = String::new();
    // file.read_to_string(&mut s)?;
}
