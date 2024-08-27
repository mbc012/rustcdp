use std::io::{BufReader, BufRead, Read};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use rand::seq::SliceRandom;
use rand::thread_rng;


use crate::chrome::browser::ChromeBrowser;


impl ChromeBrowser {
    pub(crate) fn get_port() -> Option<u16> {
        let mut ports: Vec<u16> = (9_000..11_999).collect();
        ports.shuffle(&mut thread_rng());
        ports.iter().find(|port| {
            std::net::TcpListener::bind(("127.0.0.1", **port)).is_ok()
        }).copied()
    }

    pub(crate) fn spawn_process(location: PathBuf, arguments: Vec<String>) -> Child {
        let port = match ChromeBrowser::get_port() {
            Some(val) => val,
            None => panic!("Uh oh! No ports available.")
        };
        
        let mut command_args = Vec::from(arguments);
        command_args.push(format!("--remote-debugging-port={}", port));
        
        let mut command = Command::new(location);
        let mut child = match command.args(command_args).stderr(Stdio::piped()).spawn() {
            Ok(val) => val,
            Err(_err) => panic!("Error spawning chrome child process.")
        };
        
        child
    }

    pub(crate) fn read_process(child: &mut Child) -> Option<String> {
        let stderr = child.stderr.take().expect("Failed to retrieve stderr.");
        let mut reader = BufReader::new(stderr);
        let mut dtws = "";
        let mut line = String::new();
        while reader.read_line(&mut line).expect("Failed to read line.") > 0 {
            if line.contains("DevTools listening on ") {
                let cp = line.find("ws://").unwrap();
                dtws = &line[cp..line.len()];
                break
            }
            line.clear();
        }
        if dtws.is_empty() {
            None
        } else {
            Some(dtws.to_string())
        }
    }

}

impl Drop for ChromeBrowser {
    fn drop(&mut self) {
        self.process.kill().and_then(|_| self.process.wait()).expect("Failed to teardown ChromeBrowser.");
    }
}
