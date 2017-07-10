mod board;
mod bot;
mod parser;
mod types;
mod starter_bot;


fn main() {
	let bot = starter_bot::StarterBot::new();
	let mut parser = parser::Parser::from(bot);
	parser.run();
}
