use encoding::label::encoding_from_whatwg_label;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(input: String, charset: Option<String>) -> String {
    let path = Path::new(&input);

    let mut file = match File::open(&path) {
        Err(err) => panic!("Could not open {}: {}", path.display(), err),
        Ok(file) => file,
    };

    let mut output = String::new();

    if charset.is_none() {
        match file.read_to_string(&mut output) {
            Err(err) => panic!("Could not read {}: {}", path.display(), err),
            Ok(_) => {}
        };
    } else {
        let input_charset = charset.unwrap();
        let charset = encoding_from_whatwg_label(&input_charset);

        if charset.is_none() {
            panic!("Failed to find charset {}", input_charset);
        };

        let charset = charset.unwrap();

        let mut byte_buffer: Vec<u8> = Vec::new();

        let status = file.read_to_end(&mut byte_buffer);

        if status.is_err() {
            panic!("Failed to read bytes from {}", path.display());
        };

        let string_option = charset.decode(&byte_buffer, encoding::DecoderTrap::Strict);

        if string_option.is_err() {
            panic!("Failed to convert {} to UTF-8", input_charset);
        };

        output = string_option.unwrap();
    }

    return output;
}
