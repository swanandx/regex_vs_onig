use regex::Regex;
use regex_vs_onig::URL_REGEX; 

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = &args[1];

    let re = Regex::new(URL_REGEX).unwrap();
    println!("{:?}", re.is_match(text));
}
