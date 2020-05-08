use requests::*;

fn main() {
	let boards = vec!["dinghy-blunt-hyena", "dinghy-crown-peak",  "dinghy-turbo-king", "dinghy-turbo", "tugboat-wolf"];
	let base_url = "https://landyachtz.com/ca/product/";
	let _i_hate_regex = "(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)(\\|?\\s?[A-Z]+:\\s?)(\\s[0-9]+[\\.]?[0-9]?\"\\s?)";

	let board = boards[0];
	get(format!("{}{}", base_url, board)).unwrap();
}
