extern crate clap;
use clap::{App, Arg, AppSettings};
extern crate curl;
use curl::easy::ProxyType;

pub struct GlobalOpts {
    pub hostname: String,
    pub wordlist_file: String,
    pub extensions: Vec<String>,
    pub max_threads: u16,
    pub proxy_enabled: bool,
    pub proxy_address: String,
    pub proxy_port: u16,
    pub proxy_type: ProxyType,
    pub proxy_auth_enabled: bool,
    pub proxy_username: String,
    pub proxy_password: String,   
}

pub fn get_args() -> GlobalOpts
{
    let args = App::new("dirble")
                        .version("0.1")
                        .author("Izzy Whistlecroft")
                        .about("Finds pages and folders on websites")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(Arg::with_name("wordlist")
                            .short("w")
                            .long("wordlist")
                            .value_name("wordlist")
                            .help("Sets which wordlist to use")
                            .takes_value(true)
                            .default_value("dirbcommon.txt"))
                        .arg(Arg::with_name("host")
                            .short("t")
                            .long("target")
                            .value_name("host_uri")
                            .index(1)
                            .help("The URI of the host to scan")
                            .takes_value(true)
                            .required(true)
                            .validator(starts_with_http))
                        .arg(Arg::with_name("extensions")
                            .short("X")
                            .value_name("extensions")
                            .help("Provides comma separated extensions to extend queries with")
                            .min_values(1)
                            .default_value("")
                            .value_delimiter(","))
                        .get_matches();

    // Parse the extensions into a vector, then sort it and remove duplicates
    let mut extensions = vec![String::from("")];
    for extension in args.values_of("extensions").unwrap() {
        extensions.push(String::from(extension));
    }
    extensions.sort();
    extensions.dedup();

    // Create the GlobalOpts struct and return it
    GlobalOpts {
        hostname: String::from(args.value_of("host").unwrap().clone()),
        wordlist_file: String::from(args.value_of("wordlist").unwrap().clone()),
        extensions: extensions,
        max_threads: 5,
        proxy_enabled: false,
        proxy_address: String::from(""),
        proxy_port: 0,
        proxy_type: curl::easy::ProxyType::Http,
        proxy_auth_enabled: false,
        proxy_username: String::from(""),
        proxy_password: String::from(""),   
    }
}


fn starts_with_http(hostname: String) -> Result<(), String> {
    if hostname.starts_with("https://") || hostname.starts_with("http://") {
        Ok(())
    }
    else {
        Err(String::from("The provided target URI must start with http:// or https://"))
    }
}