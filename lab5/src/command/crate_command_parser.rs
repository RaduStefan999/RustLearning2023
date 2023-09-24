use clap::Parser;
use crate::command::CommandData;
use crate::command::ConvertType;

#[derive(Parser)]
#[command(version, about = "Args paring example")]
struct CommandArgs {
    #[arg(short('i'), long("in"))]
    in_path: String,

    #[arg(short('o'), long("out"))]
    out_path: Option<String>,

    #[arg(short('e'), long("enc"), action)]
    convert_encode: bool,

    #[arg(short('d'), long("dec"), action)]
    convert_decode: bool

}

pub fn get_command() -> Option<CommandData> {
    let args = CommandArgs::parse();
    let convert_type: Option<ConvertType>;

    if args.convert_encode == false && args.convert_decode == false {
        convert_type = None;
    }
    else if args.convert_encode {
        convert_type = Some(ConvertType::Encode);
    }
    else {
        convert_type = Some(ConvertType::Decode);
    }
    
    return Some(CommandData::create(args.in_path, args.out_path, convert_type));
}