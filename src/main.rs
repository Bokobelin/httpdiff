use std::io::{stdin,stdout,Write};

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        let mut url1 = String::new();
        let mut url2 = String::new();
        print!("Please enter the 1rst URL: ");
        let _ = stdout().flush();
        stdin().read_line(&mut url1).expect("Did not enter a correct string");
        if let Some('\n') = url1.chars().next_back() {
            url1.pop();
        }
        if let Some('\r') = url1.chars().next_back() {
            url1.pop();
        }
        print!("Please enter the 2nd URL: ");
        let _ = stdout().flush();
        stdin().read_line(&mut url2).expect("Did not enter a correct string");
        if let Some('\n') = url2.chars().next_back() {
            url2.pop();
        }
        if let Some('\r') = url2.chars().next_back() {
            url2.pop();
        }
        args.insert(1, url1);
        args.insert(2, url2);
    }

    let response1 = reqwest::blocking::get(&args[1]);
    let length1 = match response1 {
        Ok(resp) => resp.text().map(|text| text.len()).unwrap_or(0),
        Err(_) => 0,
    };
    let response2 = reqwest::blocking::get(&args[2]);
    let length2 = match response2 {
        Ok(resp) => resp.text().map(|text| text.len()).unwrap_or(0),
        Err(_) => 0,
    };

    let arg1 = &args[1].to_string();
    let arg2 = &args[2].to_string();

    println!("Body Length for {arg1}: {}", length1);
    println!("Body Length for {arg2}: {}", length2);
}
