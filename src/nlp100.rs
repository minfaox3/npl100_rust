use itertools::Itertools;
use std::collections::HashMap;

pub fn nlp000() -> String {
    "stressed".chars().rev().collect()
}

pub fn nlp001() -> String {
    "パタトクカシーー".chars().enumerate().filter(|v|
        match v.0 {
            0 | 2 | 4 | 6=>true,
            _=>false
        }
    ).map(|v| v.1).collect()
}

pub fn nlp002() -> String {
    "パトカー".chars().interleave("タクシー".chars()).collect()
}

pub fn nlp003() -> Vec<i32> {
    "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."
        .replace(",","").replace(".", "").split(" ").map(|v|v.len()as i32).collect()
}

pub fn nlp004() -> HashMap<String, i32> {
    "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."
        .replace(",","").replace(".", "").split(" ").enumerate()
        .map(|v| {
            match v.0 {
                0 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 18 => (v.1.chars().nth(0).unwrap().to_string(), v.0 as i32 + 1),
                _ => (v.1[..2].to_string(), v.0 as i32 + 1)
            }
        }).collect()
}

pub fn nlp005() -> (Vec<String>, Vec<String>) {
    let s = "I am an NLPer".to_string();
    (s.chars().collect_vec().iter().tuple_windows::<(_,_)>().map(|(v1, v2)| {format!("{}{}",v1,v2)}).collect_vec(),
    s.split(" ").collect_vec().iter().tuple_windows::<(_,_)>().map(|(v1, v2)| {format!("{}{}",v1,v2)}).collect_vec())
}