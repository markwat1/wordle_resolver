use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

const MISS: u8 = 0;
const BLOW: u8 = 1;
const HIT: u8 = 2;

#[test]
fn t_check_wordle() {
    let r = check_wordle(&"test".to_string(), &"test2".to_string());
    for i in r {
        assert_eq!(i, MISS);
    }
    let r = check_wordle(&"test2".to_string(), &"test2".to_string());
    for i in r {
        assert_eq!(i, HIT);
    }
    let r = check_wordle(&"abcde".to_string(), &"etaio".to_string());
    let answer: Vec<u8> = vec![BLOW, MISS, MISS, MISS, BLOW];
    assert_eq!(r, answer);
    let r = check_wordle(&"raise".to_string(), &"roops".to_string());
    let answer: Vec<u8> = vec![HIT, MISS, MISS, BLOW, MISS];
    assert_eq!(r, answer);
    let r = check_wordle(&"raise".to_string(), &"cynic".to_string());
    let answer: Vec<u8> = vec![MISS, MISS, BLOW, MISS, MISS];
    assert_eq!(r, answer);
    let r = check_wordle(&"indol".to_string(), &"cynic".to_string());
    let answer: Vec<u8> = vec![BLOW, BLOW, MISS, MISS, MISS];
    assert_eq!(r, answer);
    let r = check_wordle(&"cutin".to_string(), &"cynic".to_string());
    let answer: Vec<u8> = vec![HIT, MISS, MISS, HIT, BLOW];
    assert_eq!(r, answer);
    let r = check_wordle(&"civic".to_string(), &"cynic".to_string());
    let answer: Vec<u8> = vec![HIT, MISS, MISS, HIT, HIT];
    assert_eq!(r, answer);
}

///
/// check_wordle
/// calculate check wordle result
///
fn check_wordle(guess: &String, word: &String) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(guess.len());
    for _i in 0..guess.len() {
        result.push(MISS);
    }
    assert_eq!(result.len(), guess.len());
    if guess.len() == word.len() {
        for (t, w) in word.chars().enumerate() {
            let mut hit: bool = false;
            let mut blow_pos = word.len();
            for (i, c) in guess.chars().enumerate() {
                if w == c {
                    if t == i {
                        result[i] = HIT;
                        hit = true;
                        break;
                    } else {
                        blow_pos = i;
                    }
                }
            }
            if hit == false && blow_pos < word.len() && result[blow_pos] == MISS {
                result[blow_pos] = BLOW;
            }
        }
    }
    //    println!("{} : {}", &guess, &word);
    //    for r in &result {
    //        print!("{}", r);
    //    }
    //    println!("");
    result
}

fn calc_weight(str: String, histgram: &HashMap<char, u64>) -> u64 {
    let mut weight_list = HashMap::new();
    for c in str.chars() {
        if c.is_alphabetic() && histgram.contains_key(&c) {
            let weight = weight_list.entry(c).or_insert(0u64);
            *weight = *histgram.get(&c).expect("notfound");
        }
    }
    let mut weight: u64 = 0;
    //    print!("{} : ", str);
    for (_k, v) in &weight_list {
        //        print!("{} + ", v);
        weight += v;
    }
    //    println!("");
    weight
}

fn minimum_weight(weights: &HashMap<&String, u64>) -> String {
    let mut min: u64 = 100000000000;
    let mut min_word = String::new();
    for (k, v) in weights {
        if *v < min {
            min = *v;
            min_word = k.to_string();
        }
    }
    min_word
}

fn maximum_weight(weights: &HashMap<&String, u64>) -> String {
    let mut max: u64 = 0;
    let mut max_word = String::new();
    for (k, v) in weights {
        if *v > max {
            max = *v;
            max_word = k.to_string();
        }
    }
    max_word
}

fn match_result(result: Vec<u8>, r: &String) -> bool {
    let mut pos = 0;
    for c in r.chars() {
        if result[pos] != c as u8 - '0' as u8 {
            return false;
        }
        pos += 1;
    }
    true
}

fn main() {
    let allwords = "five_letters.txt".to_string();
    let args: Vec<String> = env::args().collect();
    let mut result_list: Vec<(String, String)> = Vec::new();
    if args.len() > 1 {
        let result_pattern = Regex::new(r"([a-z]+):([0-2]+)").unwrap();
        for r in args {
            for cap in result_pattern.captures_iter(&r) {
                result_list.push((cap[1].to_string(), cap[2].to_string()));
            }
        }
    }

    let fs = match File::open(allwords) {
        Err(why) => panic!("Could not open {}", why),
        Ok(fs) => fs,
    };
    let mut reader = BufReader::new(fs);
    let mut line = String::new();
    let mut histgram = HashMap::new();
    let mut word_weight = HashMap::new();
    let mut words = Vec::new();
    while reader.read_line(&mut line).expect("read fail") > 0 {
        let l = line.to_lowercase().trim().to_string().clone();
        for c in l.chars() {
            if c.is_alphabetic() {
                let count = histgram.entry(c).or_insert(0u64);
                *count += 1;
            }
        }
        let mut result_ok = true;
        for r in &result_list {
            let result = check_wordle(&r.0, &l);
            if match_result(result, &r.1) != true {
                result_ok = false;
            }
        }
        if result_ok {
            words.push(l);
        }
        line.clear();
    }

    //    for (k, v) in &histgram {
    //        println!("{} : {}", v, k);
    //    }
    if words.len() > 0 {
        for w in &words {
            let weight = word_weight.entry(w).or_insert(0);
            *weight = calc_weight(w.to_string(), &histgram);
            //        println!("{} : {}", w, weight);
        }
        let min = minimum_weight(&word_weight);
        println!(
            "Minimum word : {} = {}",
            min,
            word_weight.get(&min).expect("Notfound")
        );
        let max = maximum_weight(&word_weight);
        println!(
            "Maximum word : {} = {}",
            max,
            word_weight.get(&max).expect("Notfound")
        );
    } else {
        println!("No words matches");
    }
}
