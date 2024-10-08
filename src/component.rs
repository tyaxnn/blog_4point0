use chrono::{Local,Datelike};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diary{
    pub file_name : String,
    pub title : String,
    pub date : Date,
    pub contents : String,
    pub last_editied : i64,
    pub hide : bool,
}

impl Diary {
    pub fn new() -> Diary {

        Diary{
            file_name : "".to_string(),
            title : "untitled".to_string(),
            date : Date::now(),
            contents : "".to_string(),
            last_editied : Local::now().timestamp(),
            hide : true,
        }
        
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Date{
    pub year : i32,
    pub month : u32,
    pub day : u32,
}

impl Date {
    pub fn now() -> Date {
        let now = Local::now();

        Date{
            year : now.year(),
            month : now.month(),
            day : now.day(),
        }


    }
}