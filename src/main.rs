use std::collections::HashMap;
use std::time::Duration;
use rustcdp::chrome::Chrome;
use rustcdp::chrome::UserCallMessage;
//use rustcdp::chrome::domain::target::TargetDomain;
use rustcdp::chrome::domain::target::types::{TargetID, TargetInfoTypes};

fn main () -> ! {
    let mut chrome = Chrome::new();

    let mut target_id = TargetID(String::from(""));
    let targets = chrome.get_targets().unwrap();

    for target in targets {
        println!("{}", serde_json::to_string_pretty(&target).unwrap());

        match target.r#type {
            TargetInfoTypes::Page => {
                target_id = target.target_id;
            },
            _ => {}
        }
    }


    println!("{}", target_id.0);
    let sid = chrome.attach_to_target(&target_id, false).unwrap();
    println!("{}", sid.as_str());

    loop {}
}