use shyfyr;
use clap::{Arg, App};
fn main() {
	let matches = App::new("shyfyr-client")
		.version("0.1.0")
		.about("Shyfyr encryption service")
		.author("Aydar N.")
		.arg(Arg::with_name("TEXT")
			.help("String to encrypt/decrypt")
			.required(true))
		.arg(Arg::with_name("encrypt")
			.short("e")
			.long("en")
			.help("Encrypt passed string to shyfyr"))
		.arg(Arg::with_name("decrypt")
			.short("d")
			.long("de")
			.help("Decrypt passed string from shyfyr"))
		.get_matches();
	let txt = matches.value_of("TEXT").unwrap().to_string();

    println!("{}", if matches.is_present("encrypt"){ shyfyr::ser(&txt) } else { shyfyr::de(&txt) });
}
