#[derive(PartialEq, Eq)]
pub enum ConvertType {
    Encode,
    Decode
}

pub struct CommandData {
    pub in_path: String,
    pub out_path: String,
    pub convert_type: ConvertType
}

impl CommandData {
    pub fn create(in_path: String, out_path: Option<String>, convert_type: Option<ConvertType>) -> CommandData {
        return CommandData{
            in_path: in_path,
            out_path: if out_path.is_none() {String::from(r"D:\personal\RustLabs\RustLearning2023\lab5\res\encoded_output.txt")} else {out_path.unwrap()},
            convert_type: if convert_type.is_none() {ConvertType::Encode} else {convert_type.unwrap()}
        };
    }
}

pub mod basic_command_parser;