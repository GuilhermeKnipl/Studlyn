use uuid::Uuid;



pub fn format_s(seconds: i32) -> String{
    let seconds = seconds % 60;
    format!("{:02}",seconds)
}

pub fn format_ms(seconds: i32) -> String{
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn format_hms(seconds: i32) -> String{

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn gen_uuid() -> String{
    let id = Uuid::new_v4();
    return id.to_string();
}