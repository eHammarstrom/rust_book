extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    let mut request: String = String::new();

    loop {
        io::stdout().write_all(b"Who would you like to add? (Add <name> to <department>): ");
        io::stdout().flush();

        request.clear();
        io::stdin().read_line(&mut request);

        if request.contains("quit") {
            for (k, v) in deps.iter().enumerate() {
                println!("{} : {:?}", k, v);
            }

            break;
        }

        let re = Regex::new(r"Add (.*) to (.*)").unwrap();

        if re.is_match(&request) {
            let emp_name = re.captures(&request).unwrap().get(1)
                .map_or("", |m| m.as_str());
            let emp_department = re.captures(&request).unwrap().get(2)
                .map_or("", |m| m.as_str());

            let emp_name = String::from(emp_name);

            if deps.contains_key(emp_department) {
                if let Some(v) = deps.get_mut(emp_department) {
                    v.push(emp_name);
                }
            } else {
                deps.entry(String::from(emp_department))
                    .or_insert(vec![emp_name]);
            }
        } else {
            println!("Unknown command.");
        }
    }
}
