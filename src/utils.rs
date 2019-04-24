use std::mem;

pub fn replace(data: &mut Vec<String>, from: &str, to: String) -> Option<String> {
    let index = data.iter().position(|x| x == from);
    match index {
        None => None,
        Some(i) =>  Some(mem::replace(&mut data[i], to)),
    }
}