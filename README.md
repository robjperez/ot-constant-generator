## What's this? ##

When developing client tools, you usually need to have a fresh session data to use it in your client. You can use any tool out there, but it is very annoying to copy and paste the token and the sessionId separately

This tool will generate all the code you need just to copy and paste directly into your source file.

It can generate code in Java, Swift, Objective-C and Kotlin.

To generate the session data, it uses either meet.tokbox.com or opentokrtc.tokbox.com to get it, so you can take the room and connect also with your browser.

## How to build ##

First of all, if you haven't done it yet, you will need to install Rust

`$ brew install rust`

Since in OSX 10.11 openssl is no longed included
You will need to install it via brew and add these env vars

`$ brew install openssl`

*bash*

```
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
```

*fish shell*

```
set -x OPENSSL_LIB_DIR (brew --prefix openssl)/lib
set -x OPENSSL_INCLUDE_DIR (brew --prefix openssl)/include
```

After having your environment ready, just type

`$ cargo clean && cargo install`

## How to use ##

It has two parameters
* -l | --language : [objc | swift | java | kotlin]
* -e | --environment: [meet | opentokdemo | opentokrtc (default) | opentok-meet.heroku]

You may want to pipe the output to `pbcopy` to have it automatically copied to the clipboard

```
$ ot-constant-generator -l swift
// room: 38123242-bce7-486c-8246-6a7f40196fcf
let APIKEY = "44443122"
let TOKEN = "XXX"
let SESSIONID = "XXX"

$ ot-constant-generator -l kotlin -e meet | pbcopy
//room: f0203c0c-59f9-491f-bb72-b786d2da9028
val APIKEY = "44935341";
val TOKEN = "XXX";
val SESSION_ID = "XXX";
```
