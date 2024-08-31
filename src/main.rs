use std::{fs, io};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, Write};
use std::{thread, time};
use std::path::Path;
use reqwest::Client;
use std::time::Duration;
use reqwest;
//use std::time::SystemTime; Uncomment for debug purposes

const CHECK_AT_ONCE:usize = 10;

#[tokio::main]
async fn main() 
{
    check_for_file();
    println!("\nWelcome to error/metalblaze/red lattice's sporepedia getter!");
    println!("\nInitializing...");
    //let now = SystemTime::now();
    let hashed_ids = build_id_list();
    //println!("{:?}", now.elapsed().unwrap());
    run(&hashed_ids).await;
    loop
    {
        if get_y_n_input()
        {
            run(&hashed_ids).await;
        }
        else
        {
            break;
        }
    }
    println!("\nProgram exited successfully");
    let wait_period = time::Duration::from_secs(2);
    thread::sleep(wait_period);
}

fn build_id_list() -> Vec<u32>
{
    let mut set:Vec<u32> = vec![];

    // Folder in question is id_stack
    //for line in big_list.into_iter().map(|file|BufReader::new(file).lines()).flatten()
    let lol = include_bytes!("../id_stack/all_ids_combined.txt");
    for line in lol.lines()
    {
        set.push(line.unwrap().parse().unwrap());
    }
    let lol_2 = include_bytes!("../id_stack/501_ids.txt");
    for line in lol_2.lines()
    {
        set.push(line.unwrap().parse().unwrap());
    }
    return set;
}

async fn run(valid_ids: &Vec<u32>)
{
    println!("\nPlease enter a starting ID to begin your range");
    let start = (input_value() % 10_000_000_000) as u32;
    println!("\nHow many ID's after this would you like to search? (inclusive)");
    let end = (input_value() % 10_000_000_000) as u32;
    get_range(start, end, valid_ids).await;
    println!("\nCreations successfully gathered!");
    println!("\nWould you like to search another region? (Y/N)");
}

fn check_for_file() {let _ = fs::create_dir_all("png_pile");}

fn get_y_n_input() -> bool
{
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

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
    // If this hits recursion depth, it's user error at that point lmao
    return get_y_n_input();
}

static APP_USER_AGENT: &str = "Sporepedia Archival Project";

async fn get_range(start: u32, count: u32, valid_ids: &Vec<u32>)
{
    let mut counter = 0;
    let start_index: usize = loop {
        let a = valid_ids.iter().position(|&r| r == start + counter);
        if let Some(_) = a {break a.unwrap();}
        else {counter += 1;}
    };
    counter = 0;
    let end_index: usize = loop {
        let a = valid_ids.iter().position(|&r| r == start + count + counter);
        if let Some(_) = a {break a.unwrap();}
        else {counter += 1;}
    };

    let bar = ProgressBar::new(count as u64);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.blue/cyan}] {pos:>7}/{len:7} {msg} {percent}%   Estimated time remaining: {eta}")
        .unwrap()
        .progress_chars("##-"));

    for i in (start_index..=end_index).step_by(CHECK_AT_ONCE)
    {
        //This is the format the URL's follow: http://static.spore.com/static/thumb/123/456/789/123456789123.png
        
        let urls = (i..i+CHECK_AT_ONCE).map(|k| {
            let url = url_builder(*valid_ids.get(k).unwrap());
            //println!("{url}"); // For debugging only
            (url, *valid_ids.get(k).unwrap())
        });

        let results = futures::future::join_all(urls.map(|(url, k)|
            async move 
                { 
                    let client = client_builder(); 
                    (client.expect("REASON").get(&url).send().await, url, k) 
                }
        )).await;
        
        for (result, url, k) in results.into_iter()
        {
            let result = match result {
                Ok(result) => result,
                Err(_) => 
                {
                    loop // If there was an error in fetching the request, it just retries until it works.
                    {
                        let result = reqwest::get(url.clone()).await;
                        if let Ok(result) = result {
                            break result
                        }
                    }
                }
            };

            let bytes = result.bytes().await.unwrap();
            let file_name_string = format!("png_pile//50{}.png", small_clean_2(k));
            let file_name = Path::new(&file_name_string);
            let mut file = std::fs::File::create(file_name).unwrap();
            file.write_all(&bytes).unwrap();
            bar.set_position((k - start) as u64);
        }
    }
}

fn client_builder() -> Result<Client, reqwest::Error>
{
    Client::builder().user_agent(APP_USER_AGENT).timeout(Duration::from_millis(0)).build()
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
    panic!();
}

/* Certain ID slices would have leading zeros, which would get removed when becoming a string.
   This fixes it so that it has the leading zeroes to go along with it */
fn clean_id(input: u32) -> String
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

fn small_clean(input: u32) -> String {
    if input == 1
    {
        return format!("1");
    }
    return format!("0");
}
fn small_clean_2(input: u32) -> String {
    if input / 1_000_000_000 == 0
    {
        return format!("0{input}");
    }
    return format!("{input}");
}

/* Builds a url to pull from */
fn url_builder(input: u32) -> String
{
    // First ID slice doesn't need to be cleaned because it always has a leading 5 or 3
    return format!("https://static.spore.com/static/thumb/50{}/{}/{}/50{}.png", 
        small_clean(input / 1000000000), 
        clean_id((input / 1000000) % 1000),
        clean_id((input / 1000) % 1000),
        small_clean_2(input));
}
