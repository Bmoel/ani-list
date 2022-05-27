use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Result, Seek, SeekFrom,Write};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Anime {
    pub name: String,
    pub score: f32,
    pub current_ep: i32,
    pub total_ep: i32,
    status: String,
    review: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Anime {
    pub fn new(name: String, score: f32, current_ep: i32, total_ep: i32, status: String, review: String) -> Anime {
        let created_at: DateTime<Utc> = Utc::now();
        Anime { name, score, current_ep, total_ep, status, review, created_at }
    }
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{}:\n\tScore: {}/{}\n\tProgress: {}/{}\n\tStatus: {}\n\tReview: {}\n\tEntered at: [{}]", 
                    self.name,
                    self.score,
                    10i32,
                    self.current_ep,
                    self.total_ep,
                    self.status,
                    self.review,
                    created_at)
    }
}

fn get_user_str(message: String) -> String {
    println!("{}",message);
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    user_input.pop();
    user_input
}

fn get_list(mut file: &File) -> Result<Vec<Anime>> {
    file.seek(SeekFrom::Start(0))?;
    let anilist = match serde_json::from_reader(file) {
        Ok(anilist) => anilist,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(anilist)
}

pub fn add_anime(file_name: PathBuf) -> Result<()> {
    let myfile = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;
    let mut anilist = get_list(&myfile)?;
    anilist.push(create_new_anime());
    serde_json::to_writer(myfile,&anilist)?;
    Ok(())
}

fn create_new_anime() -> Anime {
    // Get Name
    let mut user_input = String::new();
    println!("What is the name of the anime?: ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    user_input.pop();
    let ani_name: String = user_input;
    // Get Score
    let mut user_input = String::new();
    let mut ani_score: f32 = -1.0;
    let mut failed_twice: bool = false;
    while ani_score < 0.0 || ani_score > 10.0 {
        user_input.clear();
        if failed_twice {
            println!("Invalid input, please enter a floating point number between 0 and 10");
            failed_twice = false;
        } else {
            println!("What score would you like to give the anime? (0 - 10, decimals allowed): ");
        }
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");
        user_input.pop();
        let mut result  = user_input.parse();
        while result.is_err() {
            failed_twice = true;
            println!("Invalid input, please enter a floating point number between 0 and 10");
            user_input.clear();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("failed to read line");
            user_input.pop();
            result = user_input.parse();
        }
        ani_score = result.unwrap();
    }
    // Get Total Episode Count
    let mut user_input = String::new();
    let mut ani_ept: i32 = -1;
    let mut failed_twice: bool = false;
    while ani_ept < 0 {
        user_input.clear();
        if failed_twice {
            println!("Invalid input, please enter a valid integer that is greater than 0");
            failed_twice = false;
        } else {
            println!("What is the anime's total episode count? (integer only): ");
        }
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");
        user_input.pop();
        let mut result = user_input.parse();
        while result.is_err() {
            failed_twice = true;
            println!("Invalid input, please enter a valid integer that is greater than 0");
            user_input.clear();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("failed to read line");
            user_input.pop();
            result = user_input.parse();
        }
        ani_ept = result.unwrap();
        if ani_ept < 0 { failed_twice = true; }
    }
    // Get Current Episode Count
    let mut user_input = String::new();
    let mut ani_epc: i32 = -1;
    let mut failed_twice: bool = false;
    while ani_epc < 0 || ani_epc > ani_ept {
        user_input.clear();
        if failed_twice {
            println!("Invalid input, please enter a valid integer between 0 and MAX EPISODE");
            failed_twice = false;
        } else {
            println!("What episode are you currently on? (0 - MAX EPISODE, integer only): ");
        }
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");
        user_input.pop();
        let mut result = user_input.parse();
        while result.is_err() {
            failed_twice = true;
            println!("Invalid input, please enter a valid integer between 0 and MAX EPISODE");
            user_input.clear();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("failed to read line");
            user_input.pop();
            result = user_input.parse();
        }
        ani_epc = result.unwrap();
    }
    // Get Status
    let mut user_input = String::new();
    let mut ani_status: String = "".to_string();
    let mut user_num: i32 = -1;
    let mut failed: bool = false;
    while user_num < 1 || user_num > 3 {
        user_input.clear();
        if !failed {
            println!("What is the current watch status? (1 = Watching, 2 = Completed, 3 = Dropped): ");
        } else {
            println!("Invalid input, please enter an integer between 1 and 3 (1 = Watching, 2 = Completed, 3 = Dropped): ");
        }
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line");
        user_input.pop();
        user_num = user_input.parse().unwrap();
        failed = true;
    }
    if user_num == 1 {
        ani_status = "Watching".to_string();
    }
    else if user_num == 2 {
        ani_status = "Completed".to_string();
    }
    else if user_num == 3 {
        ani_status = "Dropped".to_string();
    }
    // Updating Review
    let mut user_input = String::new();
    let ani_review: String;
    println!("What is your review of the anime? (string): ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    user_input.pop();
    ani_review = user_input;
    //Return anime
    Anime::new(ani_name,ani_score,ani_epc,ani_ept,ani_status,ani_review)
}



pub fn update_anime(file_name: PathBuf) -> Result<()> {
    let myfile = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;
    let mut anilist = get_list(&myfile)?;
    let mut inc: usize = 0;
    let mut found = false;
    let name: String = get_user_str("What is the name of the anime?: ".to_string());
    for anime in anilist.clone() {
        if anime.name == name {
            anilist = ask_user(anilist.clone(),inc);
            found = true;
            break;
        }
        inc += 1;
    }
    if !found {
        println!("Anime not found in list!");
    }
    else {
        myfile.set_len(0)?;
        serde_json::to_writer(myfile,&anilist)?;
    }
    Ok(())
}

fn ask_user(anilist: Vec<Anime>, index: usize) -> Vec<Anime> {
    let mut new_list = anilist;
    // Updating Name
    let mut user_input = String::new();
    println!("Would you like to update the name? (y/n): ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    if user_input == "y\n" || user_input == "Y\n" {
        println!("New name: ");
        let mut new_name = String::new();
        std::io::stdin()
            .read_line(&mut new_name)
            .expect("failed to read line");
        new_name.pop();
        new_list[index].name = new_name;
    }
    // Updating Score
    let mut user_input = String::new();
    println!("Would you like to update the score? (y/n): ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    if user_input == "y\n" || user_input == "Y\n" {
        let mut new_score = String::new();
        let mut u_score: f32 = -1.0;
        let mut failed_twice: bool = false;
        while u_score < 0.0 || u_score > 10.0 {
            if failed_twice {
                println!("Inavlid input, please enter a floating point number between 0 and 10");
                failed_twice = false;
            } else {
                println!("New score: (0 - 10, decimals allowed)");
            }
            new_score.clear();
            std::io::stdin()
                .read_line(&mut new_score)
                .expect("failed to read line");
            new_score.pop();
            let mut result = new_score.parse();
            while result.is_err() {
                failed_twice = true;
                println!("Inavlid input, please enter a floating point number between 0 and 10");
                new_score.clear();
                std::io::stdin()
                    .read_line(&mut new_score)
                    .expect("failed to read line");
                new_score.pop();
                result = new_score.parse();
            }
            u_score = result.unwrap();
        }
        new_list[index].score = u_score;
    }
    // Updating Total Episode Count
    let mut user_input = String::new();
    println!("Would you like to update your total episode count? (y/n): ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    if user_input == "y\n" || user_input == "Y\n" {
        let mut new_ept = String::new();
        let mut u_ept: i32 = -1;
        let mut failed_twice: bool = false;
        while u_ept < 0 {
            if failed_twice {
                println!("Invalid input, please enter an integer value that is greater than 0");
                failed_twice = false;
            } else {
                println!("New total episode count: ");
            }
            new_ept.clear();
            std::io::stdin()
                .read_line(&mut new_ept)
                .expect("failed to read line");
            new_ept.pop();
            let mut result = new_ept.parse();
            while result.is_err() {
                failed_twice = true;
                println!("Invalid input, please enter an integer value that is greater than 0");
                new_ept.clear();
                std::io::stdin()
                    .read_line(&mut new_ept)
                    .expect("failed to read line");
                new_ept.pop();
                result = new_ept.parse();
            }
            u_ept = result.unwrap();
        }
        new_list[index].total_ep = u_ept;
    }
    // Updating Current Episode Count
    let mut user_input = String::new();
    println!("Would you like to update your current episode count? (y/n): ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    if user_input == "y\n" || user_input == "Y\n" {
        let mut new_epc = String::new();
        let mut u_epc: i32 = -1;
        let mut failed_twice: bool = true;
        while u_epc < 0 {
            if failed_twice {
                println!("Invalid input, please enter a value between 0 and MAX EPISODE");
                failed_twice = false;
            } else {
                println!("New episode count: ");
            }
            new_epc.clear();
            std::io::stdin()
                .read_line(&mut new_epc)
                .expect("failed to read line");
            new_epc.pop();
            let mut result = new_epc.parse();
            while result.is_err() {
                failed_twice = true;
                println!("Invalid input, please enter a value between 0 and MAX EPISODE");
                new_epc.clear();
                std::io::stdin()
                    .read_line(&mut new_epc)
                    .expect("failed to read line");
                new_epc.pop();
                result = new_epc.parse();
            }
            u_epc = result.unwrap();
        }
        new_list[index].current_ep = u_epc;
    }
    // Updating Status
    let mut user_input = String::new();
    println!("Would you like to update the status? (1 = Watching, 2 = Completed, 3 = Dropped): ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    if user_input == "y\n" || user_input == "Y\n" {
        println!("New status: ");
        let mut new_status = String::new();
        let mut u_in: i32 = -1;
        while u_in < 1 || u_in > 3 {
            std::io::stdin()
                .read_line(&mut new_status)
                .expect("failed to read line");
            new_status.pop();
            u_in = new_status.parse().unwrap();
        }
        if u_in == 1 {
            new_status = "Watching".to_string();
        }
        else if u_in == 2 {
            new_status = "Completed".to_string();
        }
        else if u_in == 3 {
            new_status = "Dropped".to_string();
        }
        new_list[index].status = new_status;
    }
    // Updating Review
    let mut user_input = String::new();
    println!("Would you like to update the review? (y/n): ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    if user_input == "y\n" || user_input == "Y\n" {
        println!("New review: ");
        let mut new_review = String::new();
        std::io::stdin()
            .read_line(&mut new_review)
            .expect("failed to read line");
        new_review.pop();
        new_list[index].review = new_review;
    }
    new_list
}

pub fn remove_anime(file_name: PathBuf) -> Result<()> {
    let myfile = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;
    let mut anilist = get_list(&myfile)?;
    let mut inc: usize = 0;
    let name = get_user_str("What is the name of the anime?: ".to_string());
    for anime in anilist.clone() {
        if anime.name == name {
            anilist.remove(inc);
            break;
        }
        inc += 1;
    }
    myfile.set_len(0)?;
    serde_json::to_writer(myfile,&anilist)?;
    Ok(())
}

pub fn search_anime(file_name: PathBuf) -> Result<()> {
    let myfile = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;
    let anilist = get_list(&myfile)?;
    let name = get_user_str("What is the name of the anime?: ".to_string());
    let mut found: bool = false;
    for anime in anilist {
        if anime.name == name {
            println!("{}",anime);
            found = true;
            break;
        }
    }
    if !found {
        println!("Anime not found in list!");
    }
    Ok(())
}

pub fn export_anime(file_name: PathBuf) -> Result<()>{
    let currentfile = OpenOptions::new()
        .read(true)
        .open(file_name)?;
    let export_file = get_user_str("What is the name of the export file?: ".to_string());
    let anilist = get_list(&currentfile)?;
    if anilist.is_empty() {
        println!("Your anime list is empty, new file not created");
    }
    else {
        let mut final_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(export_file)?;
        for anime in anilist {
            let ani_str = anime.name + &"\n".to_owned();
            final_file.write_all(ani_str.as_bytes())?;

            let score_str = "\tScore: ".to_owned() + &anime.score.to_string() + "/10\n";
            final_file.write_all(score_str.as_bytes())?;

            let ep_str = "\tProgress: ".to_owned() + &anime.current_ep.to_string() + "/" + &anime.total_ep.to_string() + "\n";
            final_file.write_all(ep_str.as_bytes())?;

            let stat_str = "\tStatus: ".to_owned() + &anime.status + "\n";
            final_file.write_all(stat_str.as_bytes())?;

            let rev_str = "\tReview: ".to_owned() + &anime.review + "\n";
            final_file.write_all(rev_str.as_bytes())?;

            let sep_str = "-----------------------------------------------------\n";
            final_file.write_all(sep_str.as_bytes())?;
        }
    }
    Ok(())
}

pub fn list_anime(file_name: PathBuf) -> Result<()> {
    let myfile = OpenOptions::new()
        .read(true)
        .open(file_name)?;
    let anilist = get_list(&myfile)?;
    if anilist.is_empty() {
        println!("Your anime list is empty!\nTry adding some using the 'add' command");
    }
    else {
        for anime in anilist {
            println!("{}",anime);
        }
    }
    Ok(())
}