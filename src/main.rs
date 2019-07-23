extern crate getopts;
extern crate uuid;
extern crate hyper;
extern crate rustc_serialize;
extern crate strfmt;

use std::collections::HashMap;
use getopts::Options;
use std::env;
use std::str::FromStr;
use uuid::Uuid;

use hyper::client::Client;
use hyper::Url;
use std::io::Read;

use std::fmt;
use std::process;

use rustc_serialize::json::Json;

use strfmt::strfmt;

enum Language {
    Swift,
    ObjC,
    Java,
    Kotlin,
    Python,
    FakePublisher,
    Csharp,
    Javascript,
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
            "fake-publisher" => Ok(Language::FakePublisher),
            "csharp" => Ok(Language::Csharp),
            "javascript" => Ok(Language::Javascript),
            _ => Err(())
        }
    }
}

impl Language {
    fn get_api_key_default_var_name(self: &Language) -> String {
        match self {
            Language::Swift | Language::ObjC => String::from_str("kApiKey").unwrap(),
            _ => String::from_str("APIKEY").unwrap(),
        }
    }

    fn get_output_format(&self) -> &'static str {
        match self {
            Language::ObjC => "// room: {room} ( {url} )\nstatic NSString* const {api_key_var_name} = @\"{api_key}\";\nstatic NSString* const kToken = @\"{token}\";\nstatic NSString* const kSessionId = @\"{session_id}\";\n",
            Language::Swift => "// room: {room} ( {url} )\nlet {api_key_var_name} = \"{api_key}\"\nlet kToken = \"{token}\"\nlet kSessionId = \"{session_id}\"\n",
            Language::Java => "//room: {room} ( {url} )\npublic static final String {api_key_var_name} = \"{api_key}\";\npublic static final String TOKEN = \"{token}\";\npublic static final String SESSION_ID = \"{session_id}\";\n",
            Language::Kotlin => "//room: {room} ( {url} )\nval {api_key_var_name} = \"{api_key}\";\nval TOKEN = \"{token}\";\nval SESSION_ID = \"{session_id}\";\n",
            Language::Python => "#room: {room} ( {url} )\n{api_key_var_name} = \"{api_key}\"\nTOKEN = \"{token}\"\nSESSION_ID = \"{session_id}\"\n",
            Language::FakePublisher => "fake-publisher -sessionId \"{session_id}\" -token \"{token}\" -apiKey \"{api_key}\"",
            Language::Csharp => "//room: {room} ( {url} )\npublic string {api_key_var_name} = \"{api_key}\";\npublic string TOKEN = \"{token}\";\npublic string SESSION_ID = \"{session_id}\";\n",
            Language::Javascript => "//room: {room} ( {url} )\nconst {api_key_var_name} = \"{api_key}\";\nconst TOKEN = \"{token}\";\nconst SESSION_ID = \"{session_id}\";\n",
        }
    }
}

#[derive(Clone, Copy)]
enum Environment {
    Meet,
    OpentokRtc,
    OpentokDemo,
    MeetHeroku
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Meet => write!(f, "meet"),
            Environment::OpentokRtc => write!(f, "opentokrtc"),
            Environment::OpentokDemo => write!(f, "opentokdemo"),
            Environment::MeetHeroku => write!(f, "opentok-meet"),
        }
    }
}

impl FromStr for Environment {
    type Err = ();
    fn from_str(s: &str) -> Result<Environment, ()> {
        match s {
            "meet" => Ok(Environment::Meet),
            "opentokrtc" => Ok(Environment::OpentokRtc),
            "opentokdemo" => Ok(Environment::OpentokDemo),
            "heroku" => Ok(Environment::MeetHeroku),
            _ => Err(())
        }
    }
}

struct SessionData {
    api_key: String,
    token: String,
    session_id: String,
    room: String,
	url: String
}

impl SessionData {
    fn get_out_string(&self, language: &Language, api_key_var_name: &String) -> String {
        let format = language.get_output_format();
        let vars: HashMap<String, String> = match language {
            Language::FakePublisher => [
                (String::from("session_id"), self.session_id.clone()),
                (String::from("token"), self.token.clone()),
                (String::from("api_key"), self.api_key.clone())].iter().cloned().collect(),
            _ => [(String::from("session_id"), self.session_id.clone()),
                (String::from("room"), self.room.clone()),
                (String::from("api_key"), self.api_key.clone()),
                (String::from("token"), self.token.clone()),
                (String::from("url"), self.url.clone()),
                (String::from("api_key_var_name"), api_key_var_name.clone())].iter().cloned().collect()
        };
        
        strfmt(&format, &vars).unwrap()
    }

    fn serialize(&self, lang: &Language, api_key_var_name: &String) -> String {
        self.get_out_string(lang, api_key_var_name)
    }

    fn new(env: &Environment, room: &String) -> Result<SessionData, &'static str> {
        let (url, room_url) = match env {
            Environment::Meet => (format!("https://meet.tokbox.com/{}", room), format!("https://meet.tokbox.com/{}", room)),
            Environment::OpentokRtc => (format!("https://opentokrtc.com/room/{}/info", room), format!("https://opentokrtc.com/room/{}", room)),
            Environment::OpentokDemo => (format!("https://opentokdemo.tokbox.com/room/{}/info", room),format!("https://opentokdemo.tokbox.com/room/{}", room)),
            Environment::MeetHeroku => (format!("https://opentok-meet.herokuapp.com/{}", room), format!("https://opentok-meet.herokuapp.com/{}", room))
        };

        let client = Client::new();
        let mut res = client.get(Url::parse(url.as_ref()).unwrap()).send().unwrap();
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();

        //println!("Response: {:?}", s);
        let data = Json::from_str(s.as_ref()).unwrap();
        let obj = data.as_object().unwrap();

        let token = match obj.get("token") {
            Some(e) => e.as_string().unwrap(),
            _ => return Err("could not parse token")
        };

        let sid = match obj.get("sessionId") {
            Some(e) => e.as_string().unwrap(),
            _ => return Err("could not parse sessionId")
        };

        let apikey = match obj.get("apiKey") {
            Some(e) => e.as_string().unwrap(),
            _ => return Err("could not parse apikey")
        };

        Ok(SessionData {
            api_key: String::from(apikey),
            token: String::from(token),
            session_id: String::from(sid),
            room: String::from(room),
			url: room_url
        })
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
    opts.reqopt("l", "language", "output language" ,"swift, objc, java, kotlin, python, fake-publisher, csharp");
    opts.optopt("e", "environment", "target env", "meet (dev), opentokrtc (prod), heroku (opentok-meet.herokuapp/prod)");
    opts.optopt("r", "room", "room name", "STRING");
    opts.optopt("a", "apikeyvar", "api key var name", "STRING");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { print!("Error: {}", f.to_string()); print_usage(&program, &opts); panic!() }
    };

    let lang = match matches.opt_str("l") {
        Some(e) => e.parse().unwrap(),
        _ => Language::Swift
    };

    let env = match matches.opt_str("e") {
        Some(e) => e.parse::<Environment>().unwrap(),
        _ => Environment::OpentokRtc
    };

    let room = match matches.opt_str("r") {
        Some(r) => r,
        _ => Uuid::new_v4().hyphenated().to_string()
    };

    let session_data = match SessionData::new(&env, &room) {
        Ok(e) => e,
        Err(e) => { eprintln!("Error with server response: {}", e); process::exit(1); }
    };

    let api_key_var_name: String = match matches.opt_str("a") {
        Some(a) => a,
        _ => lang.get_api_key_default_var_name(),
    };
    print!("{}", session_data.serialize(&lang, &api_key_var_name));
}

#[test]
fn test_session_data() {
    let env = Environment::OpentokDemo;
    let room = String::from("testotconstants");
    let s = SessionData::new(&env, &room).unwrap();
    let lang = Language::Swift;
    println!("{}", s.serialize(&lang, &lang.get_api_key_default_var_name()));
}
