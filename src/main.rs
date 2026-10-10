// feito por: maquinzz
use std::env;

const COMMON: &[&str] = &[
    "password", "123456", "12345678", "qwerty", "abc123", "monkey", "1234567",
    "letmein", "trustno1", "dragon", "baseball", "iloveyou", "master", "sunshine",
    "login", "admin", "welcome", "shadow", "123123", "654321", "superman",
];

fn entropy(p: &str) -> f64 {
    if p.is_empty() {
        return 0.0;
    }
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_symbol = false;
    for c in p.chars() {
        if c.is_ascii_lowercase() { has_lower = true; }
        else if c.is_ascii_uppercase() { has_upper = true; }
        else if c.is_ascii_digit() { has_digit = true; }
        else { has_symbol = true; }
    }
    let mut pool = 0usize;
    if has_lower { pool += 26; }
    if has_upper { pool += 26; }
    if has_digit { pool += 10; }
    if has_symbol { pool += 32; }
    let n = p.chars().count() as f64;
    n * (pool as f64).log2()
}

fn classes(p: &str) -> u32 {
    let mut c = 0;
    if p.chars().any(|x| x.is_ascii_lowercase()) { c += 1; }
    if p.chars().any(|x| x.is_ascii_uppercase()) { c += 1; }
    if p.chars().any(|x| x.is_ascii_digit()) { c += 1; }
    if p.chars().any(|x| !x.is_alphanumeric()) { c += 1; }
    c
}

fn score(p: &str) -> (u8, &'static str, Vec<&'static str>) {
    let mut issues = Vec::new();
    let lower = p.to_ascii_lowercase();
    let s;
    let e = entropy(p);
    if COMMON.contains(&lower.as_str()) {
        s = 0;
        issues.push("common password");
    } else if p.len() < 8 {
        s = 1;
        issues.push("too short");
    } else if classes(p) <= 1 {
        s = 2;
        issues.push("single character class");
    } else if e < 40.0 {
        s = 3;
        issues.push("low entropy");
    } else if e < 60.0 {
        s = 4;
        issues.push("moderate entropy");
    } else {
        s = 5;
    }
    let label = match s {
        0 => "very weak",
        1 => "weak",
        2 => "fair",
        3 => "good",
        4 => "strong",
        _ => "very strong",
    };
    (s, label, issues)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: pwcheck <password>");
        std::process::exit(1);
    }
    let p = args.join(" ");
    if p.len() > 4096 {
        eprintln!("too long");
        std::process::exit(1);
    }
    let (s, label, issues) = score(&p);
    let e = entropy(&p);
    println!("score: {}/5 ({})", s, label);
    println!("entropy: {:.1} bits", e);
    println!("classes: {}", classes(&p));
    if !issues.is_empty() {
        println!("issues:");
        for i in &issues {
            println!("- {}", i);
        }
    }
}