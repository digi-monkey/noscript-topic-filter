use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
//use wasm_bindgen::prelude::*;
//use web_sys::console;

const INITIAL_RATING: f32 = 0.5;
const SPAM_PROB_THRESHOLD: f32 = 0.8;

/// Split `msg` into a list of words.
pub fn load_word_list(msg: &str) -> Vec<String> {
    let word_list = msg.unicode_words().collect::<Vec<&str>>();
    word_list.iter().map(|word| word.to_string()).collect()
}

/// Compute the probability of each word of `msg` to be part of a spam.
pub fn rate_words(
    msg: &str,
    token_table: &HashMap<String, u32>,
    spams: &Vec<u32>,
    hams: &Vec<u32>,
) -> Vec<f32> {
    load_word_list(msg)
        .into_iter()
        .map(|word| {
            // If word was previously added in the model
            if let Some(position) = token_table.get(&word) {
                let spam: Option<&u32> = spams.get(*position as usize);
                let ham: Option<&u32> = hams.get(*position as usize);

                if spam.is_some() && ham.is_some() {
                    let spam = *spam.unwrap();
                    let ham = *ham.unwrap();
                    let spam_total_count: u32 = spams.iter().sum();
                    let ham_total_count: u32 = hams.iter().sum();
                    // If the word has only been part of spam messages,
                    // assign it a probability of 0.99 to be part of a spam
                    if spam > 0 && ham == 0 {
                        return 0.99;
                    // If the word has only been part of ham messages,
                    // assign it a probability of 0.01 to be part of a spam
                    } else if spam == 0 && ham > 0 {
                        return 0.01;
                    // If the word has been part of both spam and ham messages,
                    // calculate the probability to be part of a spam
                    } else if spam_total_count > 0 && ham_total_count > 0 {
                        let ham_prob = (ham as f32) / (ham_total_count as f32);
                        let spam_prob = (spam as f32) / (spam_total_count as f32);
                        return (spam_prob / (ham_prob + spam_prob)).max(0.01);
                    }
                }
            }
            // If word was never added to the model,
            // assign it an initial probability to be part of a spam
            INITIAL_RATING
        })
        .collect()
}

pub fn score(
    msg: &str,
    token_table: &HashMap<String, u32>,
    spams: &Vec<u32>,
    hams: &Vec<u32>,
) -> f32 {
    // Compute the probability of each word to be part of a spam
    let ratings = rate_words(msg, token_table, spams, hams);

    let ratings = match ratings.len() {
        // If there are no ratings, return a score of 0
        0 => return 0.0,
        // If there are more than 20 ratings, keep only the 10 first
        // and 10 last ratings to calculate a score
        x if x > 20 => {
            let length = ratings.len();
            let mut ratings = ratings;
            ratings.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            [&ratings[..10], &ratings[length - 10..]].concat()
        }
        // In all other cases, keep ratings to calculate a score
        _ => ratings,
    };

    // Combine individual ratings
    let product: f32 = ratings.iter().product();
    let alt_product: f32 = ratings.iter().map(|x| 1.0 - x).product();
    product / (product + alt_product)
}

pub fn identify(
    msg: &str,
    token_table: &HashMap<String, u32>,
    spams: &Vec<u32>,
    hams: &Vec<u32>,
) -> bool {
    let point = score(msg, token_table, spams, hams);
    //console::log_1(&JsValue::from_str(&format!("score point: {:#?}", point)));
    println!("score point: {:#?}", point);
    point > SPAM_PROB_THRESHOLD
}
