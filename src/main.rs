extern crate reqwest;
use std::fs;
use std::io;
use std::path::Path;
use std::{thread, time};


fn main() 
{
    check_for_file();
    println!("\nWelcome to error/metalblaze/red lattice's sporepedia getter! Please enter a starting ID to begin your range");
    run();
}

fn run()
{
    let start = input_value();
    println!("\nWhat ID would you like the search to end at? (inclusive)");
    let end = input_value();
    get_range(start, end);
    println!("\nCreations successfully gathered!");
    println!("\nWould you like to search another region? (Y/N)");
    if get_y_n_input()
    {
        println!("\nPlease enter a starting ID to begin your range");
        run();
    }
    println!("\nProgram exited successfully");
    return;
}

fn check_for_file()
{
    let _ = fs::create_dir_all("png_pile");
}

fn get_y_n_input() -> bool
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed
    {
        "Y" => return true,
        "N" => return false,
        &_ => return failed_y_n_input(),
    };
}

fn failed_y_n_input() -> bool
{
    println!("\nPlease only enter Y or N");
    return get_y_n_input();
}

fn get_range(start: u64, end: u64)
{
    for i in start..=end
    {
        // We don't need to clean the first id slice because it will always be more than 100
        let id_slice_1 = (i / 1000000000).to_string();

        let id_slice_2 = clean_id((i / 1000000) % 1000);
        let id_slice_3 = clean_id((i / 1000) % 1000);

        let file_name_string = "png_pile//".to_owned() + &i.to_string() + ".png";
        let file_name = Path::new(&file_name_string);
        //This is the format the URL's follow: http://static.spore.com/static/thumb/123/456/789/123456789123.png

        let url = "http://static.spore.com/static/thumb/".to_owned() + &id_slice_1 
            + "/" + &id_slice_2
            + "/" + &id_slice_3 
            + "/" + &i.to_string() + ".png";

        let mut file = std::fs::File::create(file_name).unwrap();

        reqwest::blocking::get(url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();

        let img_size = std::fs::metadata(file_name).unwrap().len();

        // If a png is too small, it gets deleted because it's not a real creation
        if img_size < 500
        {
            let _ = fs::remove_file(file_name);
        }

        // Rate limiting (We don't want to ddos the servers lmao)
        let wait_period = time::Duration::from_millis(50);
        thread::sleep(wait_period);
    }
}

fn input_value() -> u64
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse::<u64>()
    {
        Ok(i) =>  return i,
        Err(..) => println!("\nthis was not a valid ID: {}", trimmed),
    };
    return 500000000000;
}

fn clean_id(input: u64) -> String
{
    if input > 99
    {
        return input.to_string();
    }
    if input > 9
    {
        return "0".to_owned() + &input.to_string();
    }
    return "00".to_owned() + &input.to_string();
}