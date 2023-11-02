extern crate reqwest;
use std::fs;
use std::io;

fn main() 
{
    println!("Welcome to error/metalblaze/red lattice's sporepedia getter! Please enter a starting ID to begin your range");
    let start = input_value();
    println!("What ID would you like the search to end at? (inclusive)");
    let end = input_value();
    get_range(start, end);
    println!("Creations successfully gathered!");
}

fn get_range(start: u64, end: u64)
{
    for i in start..=end
    {
        let id_slice_1 = (i / 1000000000).to_string();
        let id_slice_2 = ((i / 1000000) % 1000).to_string();
        let id_slice_3 = ((i / 1000) % 1000).to_string();

        //http://static.spore.com/static/thumb/501/104/218/501104218057.png
        let file_name = format!("C:\\Users\\Ian\\projects\\sporepedia_getter\\png_pile\\{i}.png");

        let url = "http://static.spore.com/static/thumb/".to_owned() + &id_slice_1 
            + "/" + &id_slice_2
            + "/" + &id_slice_3 
            + "/" + &i.to_string() + ".png";

        let mut file = std::fs::File::create(file_name.clone()).unwrap();

        reqwest::blocking::get(url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();

        let img_size = std::fs::metadata(file_name.clone()).unwrap().len();

        // If a png is too small, it gets deleted because it's not a real creation
        if img_size < 1000
        {
            let _ = fs::remove_file(file_name.clone());
        }
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
        Err(..) => println!("this was not a valid ID: {}", trimmed),
    };
    return 500000000000;
}