mod bayes;
mod runtime;

use js_sys::Reflect;
//use web_sys::console;
use wasm_bindgen::prelude::*;
use std::{collections::HashMap, ops::Deref};
use once_cell::sync::Lazy;
use bayes::identify;

const  INITIAL_VECTOR_SIZE: usize = 672706;

pub static GLOBAL_TOKEN_TABLE: Lazy<std::sync::RwLock<HashMap<String, u32>>> = Lazy::new(|| {
    std::sync::RwLock::new(HashMap::new())
});

pub static GLOBAL_SPAM_VEC: Lazy<std::sync::RwLock<Vec<u32>>> = Lazy::new(|| {
    std::sync::RwLock::new(vec![0; INITIAL_VECTOR_SIZE])
});

pub static GLOBAL_HAM_VEC: Lazy<std::sync::RwLock<Vec<u32>>> = Lazy::new(|| {
    std::sync::RwLock::new(vec![0; INITIAL_VECTOR_SIZE])
});

pub fn add_to_token_table(key: &str, value: u32) {
    GLOBAL_TOKEN_TABLE.write().unwrap().insert(key.to_string(), value);
}

pub fn get_from_token_table(key: &str) -> Option<u32> {
    GLOBAL_TOKEN_TABLE.read().unwrap().get(key).cloned()
}

pub fn get_spams_vec_element(index: usize) -> Option<u32> {
    GLOBAL_SPAM_VEC.read().unwrap().get(index).cloned()
}

pub fn get_hams_vec_element(index: usize) -> Option<u32> {
    GLOBAL_HAM_VEC.read().unwrap().get(index).cloned()
}

pub fn update_spams_vec(index: usize, value: u32) {
    let mut global_vec = GLOBAL_SPAM_VEC.write().unwrap();
    if index < global_vec.len() {
        global_vec[index] = value;
    }
    // Release the write lock automatically when it goes out of scope
}

pub fn update_hams_vec(index: usize, value: u32) {
    let mut global_vec = GLOBAL_HAM_VEC.write().unwrap();
    if index < global_vec.len() {
        global_vec[index] = value;
    }
    // Release the write lock automatically when it goes out of scope
}

#[wasm_bindgen]
pub fn pre_validate(){
    let event = runtime::get_self_event().unwrap();
    let spams_str_vec = event.find_first_tag("spams").unwrap();
    let hams_str_vec = event.find_first_tag("hams").unwrap();
    let tokens: Vec<String> = event.find_first_tag("tokens").unwrap();

    let spams: Vec<u32> = spams_str_vec.into_iter().map(|s| s.parse().unwrap()).collect();
    let hams: Vec<u32> = hams_str_vec.into_iter().map(|s| s.parse().unwrap()).collect();
    
    for (index, token) in tokens.iter().enumerate() {
        add_to_token_table(token, index as u32);
    }

    for (index, count) in spams.iter().enumerate() {
        update_spams_vec(index, *count);
    }

    for (index, count) in hams.iter().enumerate() {
        update_hams_vec(index, *count);
    }
    // console::log_1(&JsValue::from_str(&format!("Global Token Table: {:?}, {:?}, {:#?}", GLOBAL_TOKEN_TABLE.read().unwrap().len(), GLOBAL_TOKEN_TABLE.read().unwrap().get("website"), tokens)));
    // console::log_1(&JsValue::from_str(&format!("Global Spam Vec: {:?}, {:?}", GLOBAL_SPAM_VEC.read().unwrap().len(), GLOBAL_SPAM_VEC.read().unwrap())));
    // console::log_1(&JsValue::from_str(&format!("Global Ham Vec: {:?}, {:?}", GLOBAL_HAM_VEC.read().unwrap().len(), GLOBAL_HAM_VEC.read().unwrap())));
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
    let mut token_table: HashMap<String, u32> = HashMap::new();
    token_table.insert("docker".to_string(), 0);
    token_table.insert("suggest".to_string(), 1);
    token_table.insert("faggot".to_string(), 2);
    let spams: Vec<u32> = vec![3, 1, 0];
    let hams: Vec<u32> = vec![0, 1, 3];
    bayes::rate_words(msg, &token_table, &spams, &hams)
}
