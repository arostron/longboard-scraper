use requests::get;
use regex::Regex;

fn main() {
	let boards = vec!["dinghy-blunt-hyena", "dinghy-crown-peak",  "dinghy-turbo-king", "dinghy-turbo", "tugboat-wolf"];
	let base_url = "https://landyachtz.com/ca/product/";
	let i_hate_regex = "(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)";


	let reee = Regex::new(i_hate_regex).unwrap();

	boards.into_iter()
	.map(|board| format!("{}{}", base_url, board))
	.map(|url| get(url)) // make get requetsts
	.filter_map(Result::ok) // ignore get reqeust errors
	.map(|result|
		// I hate this line of code so much
		result.text().unwrap().to_string()
	)
	.map(|content|
		reee.find(&content)
		.expect("no match") // theres a better way to do this too
		.as_str()
		.to_string()
	)
	.filter_map(|data| Some(data))
	.for_each(|data| println!("{}", data))
	;
}
