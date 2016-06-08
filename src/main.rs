extern crate getopts;
use getopts::Options;
use std::env;

trait Serializer {
    fn serialize<T: Serializer>(&self, session_data: &SessionData<T>) -> String;
}

struct SessionData<T: Serializer> {
    api_key: String,
    token: String,
    session_id: String,
    serializer: T
}

struct SwiftSerializer {}
struct ObjcSerializer {}

impl Serializer for SwiftSerializer {
    fn serialize<T: Serializer>(&self, session_data: &SessionData<T>) -> String {
        format!("let APIKEY={}\nlet TOKEN={}\nlet SESSIONID={}\n", 
        session_data.api_key, 
        session_data.token, 
        session_data.session_id)
    }
}

impl Serializer for ObjcSerializer {
    fn serialize<T: Serializer>(&self, session_data: &SessionData<T>) -> String {
        format!("static const NSString *APIKEY=@\"{}\"\nstatic const NSString *TOKEN=@\"{}\"\nstatic const NSString *SESSIONID=@\"{}\"\n", 
        session_data.api_key, 
        session_data.token, 
        session_data.session_id)
    }
}

impl<T: Serializer> SessionData<T> {
    fn serialize(&self) -> String {
        self.serializer.serialize(&self)
    }
    
    fn new(s: T) -> SessionData<T> {
        SessionData {
            api_key: String::from(""),
            token: String::from(""),
            session_id: String::from(""),
            serializer: s
        }
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage {} [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.reqopt("l", "language", "output language" ,"swift, objc, java, kotlin");
    opts.optopt("e", "environment", "target env", "prod, dev");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { print!("Error: {}", f.to_string()); print_usage(&program, &opts); panic!() }
    };

    let mut session_data  = match matches.opt_str("l").unwrap().as_ref() {
        "swift" => SessionData::new(SwiftSerializer{}),
        "objc" => SessionData::new(ObjcSerializer{}),
        _ => SessionData::new(SwiftSerializer{})
    };
    
    session_data.api_key = String::from("A");
    session_data.session_id = String::from("S");
    session_data.token = String::from("T");
    
    print!("{}", session_data.serialize());
}
