use std::collections::HashMap;
use std::time::Duration;
use rustcdp::chrome::Chrome;
use rustcdp::chrome::UserCallMessage;
//use rustcdp::chrome::domain::target::TargetDomain;
use rustcdp::chrome::domain::target::types::TargetID;

fn main () -> ! {
    let mut chrome = Chrome::new();

    chrome.set_discover_targets();

    std::thread::sleep(Duration::from_millis(2000));
    chrome.set_auto_attach(true, true, false);

    std::thread::sleep(Duration::from_millis(2000));
    chrome.get_targets();

    loop {}
}