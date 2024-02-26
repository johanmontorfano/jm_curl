use std::fs::OpenOptions;
use std::io::Write;

#[derive(Copy, Clone, PartialEq)]
pub enum OutputDest {
    File,
    Stdout
}

pub fn output_to_dest(dest: OutputDest, content: String, file_name: Option<String>) {
    if dest == OutputDest::File && file_name.is_some() {
        let mut file = OpenOptions::new()
            .append(true)
            .open(file_name.clone().unwrap());
        if file.is_err() {
            file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(file_name.unwrap());
        }
        file.expect("Failed to use the specified file.")
            .write(content.clone().as_bytes())
            .expect("Failed to write content to file.");
    }
    if dest == OutputDest::Stdout {
        print!("{}", content);
    }
}
