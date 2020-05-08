use requests::get;
use regex::Regex;

fn main() {
	let boards = vec!["dinghy-blunt-hyena", "dinghy-crown-peak",  "dinghy-turbo-king", "dinghy-turbo", "tugboat-wolf"];
	let base_url = "https://landyachtz.com/ca/product/";
	let i_hate_regex = "(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)";


	let reee = Regex::new(i_hate_regex).unwrap();

	boards.iter()
	.map(|board| format!("{}{}", base_url, board))
	.map(|url| get(url)) // make get requetsts
	.filter_map(Result::ok) // ignore get reqeust errors
	.map(|result| result.content()) // get body of webpage
	.filter_map(|content| Some(content)) // ignore parse fails
	.map(|content| std::str::from_utf8(content)) // convert to str
	.filter_map(Result::ok)
	.map(|content|
		reee.find(content)
		.expect("no match")
		.as_str()
	)
	.filter_map(|data| Some(data))
	.for_each(|data| println!("{}", data))
	;
}
