use std::collections::HashMap;
use std::ops::Index;
use log::info;
use serde_json::Value;
use crate::chrome::browser::message::UserCallMessage;
use crate::chrome::Chrome;
use crate::chrome::domain::target::types::{SessionID, TargetID, TargetInfo};


impl Chrome {
    pub fn activate_target(&mut self, target_id: TargetID) {
        let msg = UserCallMessage::new("Target.activateTarget")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("targetId".into(), target_id.0.into());
                hm
            });

        self.send_message(msg);
    }

    pub fn attach_to_target(&mut self, target_id: TargetID, flatten: bool) {//-> SessionID {
        let msg = UserCallMessage::new("Target.attachToTarget")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("targetId".into(), target_id.0.into());
                hm.insert("flatten".into(), flatten.into());
                hm
            });

        let idx = self.send_message(msg);
        let guard = self.user_call_registry.lock().expect("Failed to acquire UCR.");
        let res = guard.wait(idx);
        match res {
            None => panic!("No response found; timed out"),
            Some(val) => {
                let value_extract = val.iter().next().unwrap().1.clone();
                println!("{}", value_extract.to_string());
                //let sid: SessionID = value_extract.into();
                //sid
                //TODO
            }
        }


    }

    pub fn close_target(&mut self, target_id: TargetID) {
        let msg = UserCallMessage::new("Target.closeTarget")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("targetId".into(), target_id.0.into());
                hm
            });

        self.send_message(msg);
    }

    pub fn set_discover_targets(&mut self) {
        let msg = UserCallMessage::new("Target.setDiscoverTargets")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("discover".into(), true.into());
                hm
            });

        self.send_message(msg);
    }

    pub fn get_targets(&mut self) {//} -> Vec<TargetInfo> {
        //target filter Vec<HashMap<String, Value>>
        //let msg = UserCallMessage::new("Target.getTargets");
        let msg = UserCallMessage::new("Target.getTargets");

        let idx = self.send_message(msg);
        // let guard = self.user_call_registry.lock().expect("Failed to acquire UCR.");
        // let res = guard.wait(idx);
        // match res {
        //     None => panic!("No response found; timed out"),
        //     Some(val) => {
        //         let value_extract = val.iter().next().unwrap().1.clone();
        //         println!("{}", value_extract.to_string());
        //     }
        // }
    }

    pub fn set_auto_attach(&mut self, auto_attach: bool, wait_debugger: bool, flatten: bool) {
        // todo add filter

        let msg = UserCallMessage::new("Target.setAutoAttach")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("autoAttach".into(), Value::from(auto_attach));
                hm.insert("waitForDebuggerOnStart".into(), Value::from(wait_debugger));
                hm.insert("flatten".into(), Value::from(flatten));
                hm
            });

        let idx = self.send_message(msg);

    }


}


