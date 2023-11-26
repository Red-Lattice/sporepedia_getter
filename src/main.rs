use std::{fs, io};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader, Write};
use std::{thread, time};
use std::path::Path;
use ahash::AHashSet;
use std::fs::File;
use reqwest::Client;
use std::time::Duration;
use reqwest;
//use std::time::SystemTime; Uncomment for debug purposes

const CHECK_AT_ONCE:usize = 50;

#[tokio::main]
async fn main() 
{
    let mut big_list: Vec<File> = vec!();
    check_folder(Path::new("id_stack"), &mut big_list);

    check_for_file();
    println!("\nWelcome to error/metalblaze/red lattice's sporepedia getter!");
    println!("\nInitializing...");
    //let now = SystemTime::now();
    let hashed_ids = hash_ids(big_list);
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

fn check_folder(parent_folder: &Path, big_list: &mut Vec<File>)
{
    let paths = fs::read_dir(parent_folder).unwrap();

    for path in paths.into_iter().flatten().map(|dir|dir.path())
    {
        if path.is_dir()
        {
            check_folder(&path, big_list)
        }
        else
        {
            big_list.push(File::open(path).unwrap());
        }
    }
}

/// lmao
fn hash_ids(big_list: Vec<File>) -> AHashSet<u64>
{
    let mut set:AHashSet<u64> = AHashSet::new();

    // Folder in question is id_stack
    for line in big_list.into_iter().map(|file|BufReader::new(file).lines()).flatten()
    {
        set.insert(line.unwrap().parse().unwrap());
    }
    return set;
}

async fn run(valid_ids: &AHashSet<u64>)
{
    println!("\nPlease enter a starting ID to begin your range");
    let start = input_value();
    println!("\nHow many ID's after this would you like to search? (inclusive)");
    let end = input_value();
    get_range(start, end, valid_ids).await;
    println!("\nCreations successfully gathered!");
    println!("\nWould you like to search another region? (Y/N)");
    return;
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

static APP_USER_AGENT: &str = "Sporepedia Archival Team | contact at: err.error.found@gmail.com";

async fn get_range(start: u64, count: u64, valid_ids: &AHashSet<u64>)
{
    let end = start + count;

    let bar = ProgressBar::new(count);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.blue/cyan}] {pos:>7}/{len:7} {msg} {percent}%   Estimated time remaining: {eta}")
        .unwrap()
        .progress_chars("##-"));

    for i in (start..=end).step_by(CHECK_AT_ONCE)
    {
        //This is the format the URL's follow: http://static.spore.com/static/thumb/123/456/789/123456789123.png
        let mut ids_to_map: Vec<u64> = vec![];
        for j in 0..=CHECK_AT_ONCE
        {
            if valid_ids.contains(&(i + j as u64))
            {
                ids_to_map.push(i + j as u64);
            }
            else
            {
                bar.inc(1);
            }
        }
        let urls = (0..ids_to_map.len()).map(|k| {
            let url = url_builder(*ids_to_map.get(k).unwrap());
            (url, *ids_to_map.get(k).unwrap())
        });

        let results = futures::future::join_all(urls.map(|(url, k)|
            async move 
                { 
                    let client = client_builder(); 
                    (client.expect("REASON").get(url).send().await, k) 
                }
        )).await;
        
        for (result, k) in results.into_iter()
        {
            let url_end = k;
            let result = match result {
                Ok(result) => result,
                Err(_) => 
                {
                    loop // If there was an error in fetching the request, it just retries until it works.
                    {
                        let url = url_builder(k as u64 + i);
                        let result = reqwest::get(url.clone()).await;
                        if let Ok(result) = result {
                            break result
                        }
                    }
                }
            };

            let bytes = result.bytes().await.unwrap();
            // If a png is too small, it gets deleted because it's not a real creation
            if bytes.len() > 500 // Replace with 31700 if you want to filter to mainly adventures and big creations
            {
                let file_name_string = "png_pile//".to_owned() + &url_end.to_string() + ".png";
                let file_name = Path::new(&file_name_string);
                let mut file = std::fs::File::create(file_name).unwrap();
                file.write_all(&bytes).unwrap();
            }
            bar.inc(1);
        }
    }
}

fn client_builder() -> Result<Client, reqwest::Error>
{
    Client::builder().user_agent(APP_USER_AGENT).timeout(Duration::from_millis(15000)).build()
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

/* Builds a url to pull from */
fn url_builder(input: u64) -> String
{
    // First ID slice doesn't need to be cleaned because it always has a leading 5 or 3
    return "http://static.spore.com/static/thumb/".to_owned() + &(input / 1000000000).to_string() 
    + "/" + &clean_id((input / 1000000) % 1000)
    + "/" + &clean_id((input / 1000) % 1000)
    + "/" + &input.to_string() + ".png";
}