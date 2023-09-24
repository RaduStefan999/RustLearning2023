mod file_encoders;
mod command;
use crate::command::ConvertType;
use crate::command::basic_command_parser;
use crate::command::crate_command_parser;

fn main() {
    //let cmd = basic_command_parser::get_command();
    let cmd = crate_command_parser::get_command();

    if cmd.is_none() {
        println!("Failed to parse command!");
        return;
    }
    let cmd = cmd.unwrap();
    let res;

    if cmd.convert_type == ConvertType::Encode {
        res = file_encoders::encode_base_16(cmd.in_path.as_str(), cmd.out_path.as_str());
    }
    else if cmd.convert_type == ConvertType::Decode {
        res = file_encoders::decode_base_16(cmd.in_path.as_str(), cmd.out_path.as_str());
    }
    else {
        panic!("Invalid ConvertType");
    }

    match res {
        Ok(_) => println!("All good"),
        Err(err) => println!("{}", err)
    }
}
