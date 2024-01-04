mod bayes;

use js_sys::Reflect;
use web_sys::console;
use wasm_bindgen::prelude::*;
use std::{collections::HashMap, ops::Deref};
use once_cell::sync::Lazy;
use bayes::identify;

const  INITIAL_VECTOR_SIZE: usize = 5;

// Global HashMap
pub static GLOBAL_TOKEN_TABLE: Lazy<std::sync::RwLock<HashMap<String, u64>>> = Lazy::new(|| {
    std::sync::RwLock::new(HashMap::new())
});

// Global Vec
pub static GLOBAL_SPAM_VEC: Lazy<std::sync::RwLock<Vec<u32>>> = Lazy::new(|| {
    std::sync::RwLock::new(vec![0; INITIAL_VECTOR_SIZE])
});

// Global Vec
pub static GLOBAL_HAM_VEC: Lazy<std::sync::RwLock<Vec<u32>>> = Lazy::new(|| {
    std::sync::RwLock::new(vec![0; INITIAL_VECTOR_SIZE])
});

#[wasm_bindgen]
pub fn add_to_token_table(key: &str, value: u64) {
    GLOBAL_TOKEN_TABLE.write().unwrap().insert(key.to_string(), value);
}

#[wasm_bindgen]
pub fn get_from_token_table(key: &str) -> Option<u64> {
    GLOBAL_TOKEN_TABLE.read().unwrap().get(key).cloned()
}

#[wasm_bindgen]
pub fn get_spams_vec_element(index: usize) -> Option<u32> {
    GLOBAL_SPAM_VEC.read().unwrap().get(index).cloned()
}

#[wasm_bindgen]
pub fn get_hams_vec_element(index: usize) -> Option<u32> {
    GLOBAL_HAM_VEC.read().unwrap().get(index).cloned()
}

#[wasm_bindgen]
pub fn update_spams(index: usize, value: u32) {
    let mut global_vec = GLOBAL_SPAM_VEC.write().unwrap();
    if index < global_vec.len() {
        global_vec[index] = value;
    }
    // Release the write lock automatically when it goes out of scope
}

#[wasm_bindgen]
pub fn update_hams(index: usize, value: u32) {
    let mut global_vec = GLOBAL_HAM_VEC.write().unwrap();
    if index < global_vec.len() {
        global_vec[index] = value;
    }
    // Release the write lock automatically when it goes out of scope
}

#[wasm_bindgen]
pub fn pre_validate(){
    let spams: Vec<u32> = vec![3, 1, 0];
    let hams: Vec<u32> = vec![0, 1, 3];
    let tokens: Vec<String> = vec!["docker".to_string(), "suggest".to_string(), "faggot".to_string()];

    for (index, token) in tokens.iter().enumerate() {
        add_to_token_table(token, index as u64);
    }

    for (index, count) in spams.iter().enumerate() {
        update_spams(index, *count);
    }

    for (index, count) in hams.iter().enumerate() {
        update_hams(index, *count);
    }

    // Print debug information
    console::log_1(&JsValue::from_str(&format!("Global Token Table: {:?}", GLOBAL_TOKEN_TABLE.read().unwrap())));
    console::log_1(&JsValue::from_str(&format!("Global Spam Vec: {:?}", GLOBAL_SPAM_VEC.read().unwrap())));
    console::log_1(&JsValue::from_str(&format!("Global Ham Vec: {:?}", GLOBAL_HAM_VEC.read().unwrap())));
}

#[wasm_bindgen]
pub fn is_valid_event(event: JsValue) -> bool {
    if let Some(obj) = event.dyn_ref::<js_sys::Object>() {
        if let Ok(content) = Reflect::get(obj, &JsValue::from_str("content")) {
            if let Some(content) = content.as_string() {
                let is_spam = identify(content.as_str(), GLOBAL_TOKEN_TABLE.read().unwrap().deref(), GLOBAL_SPAM_VEC.read().unwrap().deref(), GLOBAL_HAM_VEC.read().unwrap().deref());
                if is_spam {
                    return true;
                }
            }
        }
    }
    false
}

#[wasm_bindgen]
pub fn rating(msg: &str)-> Vec<f32>{
    let mut token_table: HashMap<String, u64> = HashMap::new();
    token_table.insert("docker".to_string(), 0);
    token_table.insert("suggest".to_string(), 1);
    token_table.insert("faggot".to_string(), 2);
    let spams: Vec<u32> = vec![3, 1, 0];
    let hams: Vec<u32> = vec![0, 1, 3];
    //bayes::rate_words(msg, GLOBAL_TOKEN_TABLE.read().unwrap().deref(), GLOBAL_SPAM_VEC.read().unwrap().deref(), GLOBAL_HAM_VEC.read().unwrap().deref())
    bayes::rate_words(msg, &token_table, &spams, &hams)
}
