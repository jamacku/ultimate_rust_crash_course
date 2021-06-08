pub fn inspect(argument: &String) -> bool {
    argument.ends_with("s")
}

pub fn change(argument: &mut String) {
    if !inspect(&argument) {
        argument.push_str("s");
    }
}

pub fn eat(argument: &String) -> bool {
    argument.starts_with("b") && argument.contains("a")
}

pub fn add(x: &i32, y: &i32) -> i32 {
    (*x) + (*y)
}
