use reqwest;
use sha1::{Digest, Sha1};
use hex;

// tasty chatgpt code. 
fn main() {
    let input_string = "12345";
    
    let sha1_hash = sha1_string(input_string);
    println!("SHA-1 hash: {} of: \"{}\"", sha1_hash, &input_string);


    if let Err(err) = tokio::runtime::Runtime::new().unwrap().block_on(check_hash(&sha1_hash)) {
        eprintln!("Error: {}", err);
    }
}

async fn check_hash(sha1_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hash_prefix = &sha1_hash[..5]; // take the first 5 characters of the hash so its still a super secret hash
    let api_url = format!("https://api.pwnedpasswords.com/range/{}", hash_prefix);

    let response = reqwest::get(&api_url).await?.text().await?;
    let lines: Vec<&str> = response.lines().collect();

    let suffix = &sha1_hash[5..].to_uppercase();
    let mut found = false;

    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 && parts[0] == suffix {
            let count = parts[1];
            println!("This hash has been pwned {} times.", count);
            found = true;
            break;
        }
    }

    if !found {
        println!("This hash has not been pwned!");
    }

    Ok(())
}


fn sha1_string(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}
