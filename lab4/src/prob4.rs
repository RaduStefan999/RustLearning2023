use std::{io, fs};



fn read_and_format_hosts_file(file_path: &str) -> Result<Vec<(String, String)>, io::Error>
{
    let read_file_res = fs::read_to_string(file_path)?;

    let mut result: Vec<(String, String)> = Vec::new();

    for line in read_file_res.lines()
    {
        if line.starts_with("#")
        {
            continue;
        }
        
        let res: Vec<&str> = line.split(" ").filter(|it| it.len() > 0).collect();
        if res.len() >= 2
        {
            result.push((String::from(res[0]), String::from(res[1])));
        }
    }

    return Ok(result);
}

pub fn prob4_start()
{
    let result = read_and_format_hosts_file(r"C:\Windows\System32\drivers\etc\services");

    let result_ok = match result {
        Ok(res) => res,
        Err(err) => { println!("{:?}", err); return; }
    };

    for (ip, host) in result_ok
    {
        println!("{} => {}", ip, host);
    }
}