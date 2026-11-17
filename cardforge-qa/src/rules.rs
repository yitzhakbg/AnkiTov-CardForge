//! Minimal reader for specs/qa/bar1-rules.yaml.
//!
//! Only numeric thresholds are read, so the gate cannot hardcode a number the rules
//! file already declares. Booleans and strings stay descriptive in the file.
//! No external YAML dependency: this is a flat key/value reader with indent paths.

use std::collections::BTreeMap;

pub struct Rules {
    pub version: u32,
    pub exit_clean: u32,
    pub exit_fails: u32,
    pub exit_warns: u32,
    pub vals: BTreeMap<String, f64>,
}

fn num(s: &str) -> Option<f64> {
    s.trim().trim_matches('"').parse::<f64>().ok()
}

pub fn load(path: &str) -> Rules {
    let text = std::fs::read_to_string(path).expect("rules file not found");
    let mut vals: BTreeMap<String, f64> = BTreeMap::new();
    let mut stack: Vec<(usize, String)> = Vec::new();

    for raw in text.lines() {
        let line = raw.trim_end();
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        while let Some(&(l, _)) = stack.last() {
            if l >= indent {
                stack.pop();
            } else {
                break;
            }
        }
        let (key, value) = match t.split_once(':') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => continue,
        };
        if value.is_empty() {
            stack.push((indent, key.to_string()));
            continue;
        }
        if value.starts_with('{') {
            let inner = value.trim_matches(|c| c == '{' || c == '}');
            for pair in inner.split(',') {
                if let Some((k2, v2)) = pair.split_once(':') {
                    if let Some(n) = num(v2) {
                        let mut p: Vec<String> = stack.iter().map(|(_, k)| k.clone()).collect();
                        p.push(key.to_string());
                        p.push(k2.trim().to_string());
                        vals.insert(p.join("."), n);
                    }
                }
            }
            continue;
        }
        if let Some(n) = num(value) {
            let mut p: Vec<String> = stack.iter().map(|(_, k)| k.clone()).collect();
            p.push(key.to_string());
            vals.insert(p.join("."), n);
        }
    }

    Rules {
        version: vals.get("rules_version").copied().unwrap_or(0.0) as u32,
        exit_clean: vals.get("exit_codes.clean").copied().unwrap_or(0.0) as u32,
        exit_fails: vals.get("exit_codes.fails").copied().unwrap_or(1.0) as u32,
        exit_warns: vals.get("exit_codes.warns").copied().unwrap_or(2.0) as u32,
        vals,
    }
}

impl Rules {
    pub fn get(&self, path: &str) -> Option<f64> {
        self.vals.get(path).copied()
    }
}
