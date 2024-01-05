use crate::types::Event;
use csv;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Counter {
    ham: u32,
    spam: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Train {
    token_table: HashMap<String, Counter>,
}

impl Train {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn train(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(path)?;
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let msg = record.get(1).unwrap();
                    if msg.eq("4") {
                        // computer & science class
                        let train_msg = record.get(2).unwrap();
                        self.train_spam(train_msg);

                        let s1 = record.get(2).unwrap();
                        self.train_spam(s1);

                        let s2 = record.get(3).unwrap();
                        self.train_spam(s2);
                    } else {
                        let train_msg = record.get(2).unwrap();
                        self.train_ham(train_msg);

                        let s1 = record.get(2).unwrap();
                        self.train_ham(s1);

                        let s2 = record.get(3).unwrap();
                        self.train_ham(s2);
                    }
                }
                Err(err) => println!("{:#?}", err),
            }
        }

	println!("trained, total tokens: {:#?}", self.token_table.len());
        Ok(())
    }

    pub fn save_vecs(&self) -> Result<(), io::Error> {
        let mut spams: Vec<u32> = vec![];
        let mut hams: Vec<u32> = vec![];
        let mut words: Vec<String> = vec![];
        for (key, value) in self.token_table.iter() {
            spams.push(value.spam);
            hams.push(value.ham);
            words.push(key.to_string());
        }

        let spam_file = File::create("spam-vec.txt")?;
        let ham_file = File::create("ham-vec.txt")?;
        let word_file = File::create("tokens.txt")?;

        to_writer(spam_file, &spams)?;
        to_writer(ham_file, &hams)?;
        to_writer(word_file, &words)?;

        Ok(())
    }

    pub fn from_local_vecs_to_event() -> Result<(), io::Error> {
        let spams = read_vec("spam-vec.txt").unwrap();
        let hams = read_vec("ham-vec.txt").unwrap();
        let token_vec = read_tokens("tokens.txt").unwrap();

        let file: File = File::create("algo_event.json")?;

        let value: Event = Event {
            content: "".to_string(),
            created_at: 1704354395,
            id: "a4e602e7ebb85bc3d3eae64476db5e2987d8370fc18f2b467b5c4c71fa8671da".to_string(),
            kind: 32024,
            pubkey: "8fb140b4e8ddef97ce4b821d247278a1a4353362623f64021484b372f948000c".to_string(),
            sig: "cd986256fbdcc32b8064dd09bb2efc57e1d06914b8c4721b0524d35d70d59ccb471ea2df64c9c1cf47bb4b188c25718dbd56ce110073e563b326f0463b291fe1".to_string(),
            tags: vec![spams.iter().map(|v|v.to_string()).collect(), hams.iter().map(|v|v.to_string()).collect(), token_vec],
          };

        to_writer(file, &value)?;

        Ok(())
    }

    pub fn save_event(&self) -> Result<(), io::Error> {
        let mut spams: Vec<String> = vec!["spams".to_string()];
        let mut hams: Vec<String> = vec!["hams".to_string()];
        let mut words: Vec<String> = vec!["tokens".to_string()];

        for (key, value) in self.token_table.iter() {
            let spam = value.spam.to_string();
            let ham = value.ham.to_string();
            let token = key.to_string();
            spams.push(spam);
            hams.push(ham);
            words.push(token);
        }

        let file: File = File::create("algo_event.json")?;

        let value: Event = Event {
            content: "".to_string(),
            created_at: 1704354395,
            id: "a4e602e7ebb85bc3d3eae64476db5e2987d8370fc18f2b467b5c4c71fa8671da".to_string(),
            kind: 32024,
            pubkey: "8fb140b4e8ddef97ce4b821d247278a1a4353362623f64021484b372f948000c".to_string(),
            sig: "cd986256fbdcc32b8064dd09bb2efc57e1d06914b8c4721b0524d35d70d59ccb471ea2df64c9c1cf47bb4b188c25718dbd56ce110073e563b326f0463b291fe1".to_string(),
            tags: vec![spams, hams, words],
          };

        to_writer(file, &value)?;

        Ok(())
    }

    pub fn from_local_vecs_to_args() -> Result<(HashMap<String, u32>, Vec<u32>, Vec<u32>), std::io::Error> {
        let spams = read_vec("spam-vec.txt").unwrap();
        let hams = read_vec("ham-vec.txt").unwrap();
        let token_vec = read_tokens("tokens.txt").unwrap();
        let mut tokens: HashMap<String, u32> = HashMap::new();

        for (index, token) in token_vec.iter().enumerate() {
            tokens.insert(token.to_string(), index as u32);
        }

	Ok((tokens, spams, hams))
    }

    /// Split `msg` into a list of words.
    fn load_word_list(msg: &str) -> Vec<String> {
        let word_list = msg.unicode_words().collect::<Vec<&str>>();
        word_list.iter().map(|word| word.to_string()).collect()
    }

    /// Train the classifier with a spam `msg`.
    pub fn train_spam(&mut self, msg: &str) {
        for word in Self::load_word_list(msg) {
            let counter = self.token_table.entry(word).or_default();
            counter.spam += 1;
        }
    }

    /// Train the classifier with a ham `msg`.
    pub fn train_ham(&mut self, msg: &str) {
        for word in Self::load_word_list(msg) {
            let counter = self.token_table.entry(word).or_default();
            counter.ham += 1;
        }
    }
}

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
