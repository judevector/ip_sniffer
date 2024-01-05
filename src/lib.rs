use std::net::IpAddr;
use std::str::FromStr;

pub struct Arguments {
    pub flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

mod utils {}

impl Arguments {
    pub fn new(args: &[String]) -> std::result::Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = &args[1];
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = &args[1];
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want
                \r\n -h or -help to show this help message"
                );
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid ip address; must be IPV4 or IPV6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Arguments {
                    threads,
                    flag: flag.to_string(), // Convert `&String` to `String`
                    ipaddr,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}
