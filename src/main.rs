extern crate reqwest;
use std::fs;
use std::io;

fn main() 
{
    //get_creator("MaxisCactus");
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
        // We don't need to clean the first id slice because it will always be more than 100
        let id_slice_1 = (i / 1000000000).to_string();

        let id_slice_2_int = (i / 1000000) % 1000;
        let id_slice_3_int = (i / 1000) % 1000;

        let id_slice_2 = clean_id(id_slice_2_int);
        let id_slice_3 = clean_id(id_slice_3_int);

        //This is the format the URL's follow: http://static.spore.com/static/thumb/123/456/789/123456789123.png
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
        if img_size < 500
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

fn get_creator(u_id: &str)
{
    let file_name = format!("C:\\Users\\Ian\\projects\\sporepedia_getter\\png_pile\\u_id.png");

    let url = "http://www.spore.com/rest/assets/user/MaxisCactus/0/3";

    let mut file = std::fs::File::create(file_name.clone()).unwrap();

    reqwest::blocking::get(url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();

    //println!("Creator: {}", file.to_string());
}