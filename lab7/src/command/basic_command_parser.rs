use std::env;
use crate::command::CommandData;
use crate::command::ConvertType;

pub fn get_command() -> Option<CommandData> {
    let mut in_path: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut convert_type: Option<ConvertType> = None;

    let stored_arguments: Vec<String> = env::args().into_iter().skip(1).collect();

    if !(stored_arguments.len() > 1 && stored_arguments.len() < 6) {
        println!("Invalid nr of arguments");
        return None;
    }

    let mut skip = false;

    for (index, argument) in stored_arguments.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }

        if argument == "--in" {
            if index + 1 >= stored_arguments.len() {
                println!("No argument after --in");
                return None;
            }
            in_path = Some(stored_arguments[index + 1].clone());
            skip = true;
            continue;
        }
        if argument == "--out" {
            if index + 1 >= stored_arguments.len() {
                println!("No argument after --out");
                return None;
            }
            out_path = Some(stored_arguments[index + 1].clone());
            skip = true;
            continue;
        }
        if argument == "--dec" {
            convert_type = Some(ConvertType::Decode);
            continue;
        }
        if argument == "--enc" {
            convert_type = Some(ConvertType::Encode);
            continue;
        }
    }

    if in_path.is_none() {
        println!("No --in parameter");
        return None;
    }

    return Some(CommandData::create(in_path.unwrap(), out_path, convert_type));
}