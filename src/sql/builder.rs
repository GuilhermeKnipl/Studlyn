use rusqlite::Connection;
use directories::ProjectDirs;
use std::{fs::{self, File},path::PathBuf, process::exit};



pub fn get_path() -> PathBuf {
    let proj_dir = ProjectDirs::from("com", "lynx" , "Studlyn")
        .expect("Failed to find config dir");

    let dir_data = proj_dir.data_dir();
    fs::create_dir_all(dir_data).unwrap();
    let db_path = dir_data.join("studlyn.db");
    if db_path.exists(){

        return db_path;
    }
    else{
        match File::create(&db_path){
            Ok(_file)=>{}
            Err(e) => {
                println!("{e}");
                exit(1);
            }
        }

        create_tables(&db_path);

        return db_path;
    }

}

fn fast_connection(path: &PathBuf) -> Connection{
    let conn = Connection::open(path).unwrap();
    return conn
}


pub fn establish_connection() -> Connection{
    let db_path = get_path();
    let conn = Connection::open(db_path).unwrap(); 
    return conn
}

pub fn create_tables(path: &PathBuf) {
    let conn = fast_connection(path);

    // `activity_log` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity_log (
            idx INTEGER PRIMARY KEY AUTOINCREMENT,
            creation_date DATE NOT NULL,
            activity_duration TIME NOT NULL,
            main_category VARCHAR(60) NOT NULL,
            category VARCHAR(60),
            difficult INTEGER,
            start_time TIME NOT NULL,
            end_time TIME NOT NULL,
            timer_type TEXT NOT NULL
            key_pk TEXT NOT NULL
        )",
        [],
    ).unwrap();

    // `activity_infos` table
        conn.execute(
            //uuid shared between tables
        "CREATE TABLE IF NOT EXISTS activity_info (
       
            idx INTEGER PRIMARY KEY AUTOINCREMENT,
            key_fk TEXT NOT NULL,
            work_time TIME NOT NULL,
            break_time TIME NOT NULL,
            )",
        [],
    ).unwrap();
}

