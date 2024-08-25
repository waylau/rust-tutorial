 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://waylau.com/data/people.json")?;
    let body = response.text()?;
    println!("{}", body);
    Ok(())
}