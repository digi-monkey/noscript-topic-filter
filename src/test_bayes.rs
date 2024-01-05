mod bayes;
use serde_json::from_reader;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::collections::HashMap;

pub fn read_vec(file: &str) -> Result<Vec<u32>, io::Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let vec_data: Vec<u32> = from_reader(reader)?;
    Ok(vec_data)
}

pub fn read_tokens(file: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let vec_data: Vec<String> = from_reader(reader)?;
    Ok(vec_data)
}

fn main() -> Result<(), std::io::Error> {
    let spams = read_vec("spam-vec.txt").unwrap();
    //println!("load vec, len: {:#?}, [4]:{:#?}", spams.len(), spams.get(4));

    let hams = read_vec("ham-vec.txt").unwrap();
    //println!("load vec, len: {:#?}, [4]:{:#?}", hams.len(), hams.get(4));

    let token_vec = read_tokens("tokens.txt").unwrap();
    // println!(
    //     "load vec, len: {:#?}, [4]:{:#?}",
    //     token_vec.len(),
    //     token_vec.get(4)
    // );
    let mut tokens: HashMap<String, u64> = HashMap::new();

    for (index, token) in token_vec.iter().enumerate() {
	tokens.insert(token.to_string(), index as u64);
    }

    // Print debug information
    let test_word = "Content";
    let pos = tokens.get(test_word).unwrap(); 
    let spam = spams.get(*pos as usize).unwrap();
    let ham = hams.get(*pos as usize).unwrap();
    println!("test_word {:?}: {:?}, {:?}", test_word, spam, ham);

    let content = "Noticed he said CONTENT.  Content is information aside from the website design and layout.\n\nContent on a website is best unique, as opposed to syndicated (taken from someone else).\n\nIf your site is uniquely written, that's good.  If it's well written, that's great.\n\nIf your site is writing about an interesting topic, uniquely written by you, and well-written, that's awesome -- That's the premisis for creating content for a good website.\n\nAlso, content can come in forms of raw data -- Like, if you wanted to make a travel site -- you can list the prices it takes to fly from Chicago to Pittsburg.  Or you can have a currency converter.\n\nAnother thing you can incorporate is a chatroom or message board for users to discuss the content and post their suggestions.";
    let result = bayes::identify(content, &tokens, &spams, &hams);
    println!("is spam: {:#?}", result);
    Ok(())
}
