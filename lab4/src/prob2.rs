use serde::Deserialize;

#[derive(Deserialize)]
struct EmojiData
{
    name: String,
    category: String,
    group: String,
    htmlCode: Vec<String>,
    unicode: Vec<String>
}

fn get_emoji_data_raw(remote_uri: &str) -> Result<String, std::io::Error>
{
    let resp = ureq::get(remote_uri).call().into_string()?;
    return Ok(resp);
}

fn get_emoji_data(remote_uri: &str) -> Option<Vec<EmojiData>>
{
    let data = get_emoji_data_raw(remote_uri);
    if let Err(err) = data
    {
        println!("Io error");
        return None;
    }
    let data = data.unwrap();

    let deserialized_data:  Result<Vec<EmojiData>, _> = serde_json::from_str(&data);
    if let Err(err) = deserialized_data
    {
        println!("Format error");
        return None;
    }

    return Some(deserialized_data.unwrap());
}

fn categorize(data: &Vec<EmojiData>)
{

}

fn prob2_start()
{
    if let Some(data) = get_emoji_data("http://127.0.0.1/get_data")
    {
        categorize(&data);
    }
}