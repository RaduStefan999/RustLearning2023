use std::{fs, io};

use serde_json::value::Value;

use actix_web::{
    get,
    web::Json
};

#[get("/get_data")]
pub async fn get_data() -> Json<Vec<Value>> {
    let data = fs::read_to_string(r"D:\personal\RustLabs\RustLearning2023\lab4_rust_service\src\data\all.json");

    if let Ok(read_json) = data
    {
        if let Ok(deserialized_json) = serde_json::from_str(read_json.as_str())
        {
            return Json(deserialized_json);
        }
    }

    return Json(vec![]);
}