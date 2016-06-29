extern crate getopts;
extern crate uuid;
extern crate hyper;
extern crate rustc_serialize;

use getopts::Options;
use std::env;
use std::str::FromStr;
use uuid::Uuid;

use hyper::client::Client;
use hyper::Url;
use std::io::Read;

use rustc_serialize::json::Json;

enum Language {
    Swift,
    ObjC,
    Java,
    Kotlin,
    Python
}

impl FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Language, ()> {
        match s {
            "swift" => Ok(Language::Swift),
            "objc" => Ok(Language::ObjC),
            "java" => Ok(Language::Java),
            "kotlin" => Ok(Language::Kotlin),
            "python" => Ok(Language::Python),
            _ => Err(())
        }
    }
}

enum Environment {
    Meet,
    OpentokRtc
}

impl FromStr for Environment {
    type Err = ();
    fn from_str(s: &str) -> Result<Environment, ()> {
        match s {
            "meet" => Ok(Environment::Meet),
            "opentokrtc" => Ok(Environment::OpentokRtc),
            _ => Err(())
        }
    }
}

struct SessionData {
    api_key: String,
    token: String,
    session_id: String,
    room: String
}

impl SessionData {
    fn get_out_string(&self, language: &Language) -> String {
        match *language {
            Language::ObjC =>
            format!("// room: {}\nstatic NSString* const kApiKey = @\"{}\";\nstatic NSString* const kToken = @\"{}\";\nstatic NSString* const kSessionId = @\"{}\";\n",
            self.room,
            self.api_key,
            self.token,
            self.session_id),
            Language::Swift =>
            format!("// room: {}\nlet APIKEY = \"{}\"\nlet TOKEN = \"{}\"\nlet SESSIONID = \"{}\"\n",
            self.room,
            self.api_key,
            self.token,
            self.session_id),
            Language::Java =>
            format!("//room: {}\npublic static final String APIKEY = \"{}\";\npublic static final String TOKEN = \"{}\";\npublic static final String SESSION_ID = \"{}\";\n",
            self.room,
            self.api_key,
            self.token,
            self.session_id),
            Language::Kotlin =>
            format!("//room: {}\nval APIKEY = \"{}\";\nval TOKEN = \"{}\";\nval SESSION_ID = \"{}\";\n",
            self.room,
            self.api_key,
            self.token,
            self.session_id),
            Language::Python =>
            format!("#room: {}\nAPIKEY = \"{}\"\nTOKEN = \"{}\"\nSESSION_ID = \"{}\"\n",
            self.room,
            self.api_key,
            self.token,
            self.session_id)
        }
    }
    fn serialize(&self, lang: &Language) -> String {
        self.get_out_string(lang)
    }

    fn new(env: &Environment, room: &String) -> SessionData {

        let url = match *env {
            Environment::Meet => format!("https://meet.tokbox.com/{}", room),
            Environment::OpentokRtc => format!("https://opentokrtc.com/{}.json", room)
        };
        //println!(">>> {}", url);

        let client = Client::new();
        let mut res = client.get(Url::parse(url.as_ref()).unwrap()).send().unwrap();
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();

        //println!("Response: {:?}", s);
        let data = Json::from_str(s.as_ref()).unwrap();
        let obj = data.as_object().unwrap();
        let token = obj.get("token").unwrap().as_string().unwrap();
        let sid = obj.get("sessionId").unwrap().as_string().unwrap();
        let apikey = obj.get("apiKey").unwrap().as_string().unwrap();

        SessionData {
            api_key: String::from(apikey),
            token: String::from(sid),
            session_id: String::from(token),
            room: String::from(room.as_ref())
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
    opts.reqopt("l", "language", "output language" ,"swift, objc, java, kotlin, python");
    opts.optopt("e", "environment", "target env", "meet, opentokrtc");
    opts.optopt("r", "room", "room name", "STRING");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { print!("Error: {}", f.to_string()); print_usage(&program, &opts); panic!() }
    };

    let env = match matches.opt_str("e") {
        Some(e) => e.parse::<Environment>().unwrap(),
        _ => Environment::OpentokRtc
    };

    let room = match matches.opt_str("r") {
        Some(r) => r,
        _ => Uuid::new_v4().hyphenated().to_string()
    };

    let session_data = SessionData::new(&env, &room);
    let lang = matches.opt_str("l").unwrap().parse::<Language>().unwrap();
    print!("{}", session_data.serialize(&lang));
}
