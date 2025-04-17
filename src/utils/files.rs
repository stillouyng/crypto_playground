use std::{fs::File, io::Write};

pub fn write_to_file(data: &[u8], filename: &str) -> bool {
    let filepath = format!("files/{}.txt", filename);
    let mut file = File::create(filepath).unwrap();
    let string_binary = get_binary(data);
    file.write_all(string_binary.as_bytes()).unwrap();
    true
}

pub fn get_binary(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("{:08b}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}
