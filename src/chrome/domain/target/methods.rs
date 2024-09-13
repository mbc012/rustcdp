use std::collections::HashMap;
use std::ops::Index;
use log::info;
use serde_json::Value;
use crate::chrome::browser::message::UserCallMessage;
use crate::chrome::Chrome;
use crate::chrome::domain::target::types::{SessionID, TargetID, TargetInfo};
use crate::error::{Result, Error};

impl Chrome {
    pub fn activate_target(&mut self, target_id: &TargetID) {
        let msg = UserCallMessage::new("Target.activateTarget")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("targetId".into(), target_id.into());
                hm
            });

        self.send_message(msg);
    }

    pub fn attach_to_target(&mut self, target_id: &TargetID, flatten: bool) -> Result<SessionID> {
        let msg = UserCallMessage::new("Target.attachToTarget")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("targetId".into(), target_id.into());
                hm.insert("flatten".into(), flatten.into());
                hm
            });

        let id = self.send_message(msg);

        let val = self.wait_ucr(id)?
            .get("sessionId").
            ok_or(Error::ValueNotFound)?
            .clone();

        Ok(serde_json::from_value(val)?)
    }

    pub fn close_target(&mut self, target_id: &TargetID) {
        let msg = UserCallMessage::new("Target.closeTarget")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("targetId".into(), target_id.into());
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

    pub fn get_targets(&mut self) -> Result<Vec<TargetInfo>> {
        //target filter Vec<HashMap<String, Value>>
        let msg = UserCallMessage::new("Target.getTargets");
        let id = self.send_message(msg);

        let res = self.wait_ucr(id)?
            .get("targetInfos")
            .ok_or(Error::ValueNotFound)?
            .clone();

        Ok(serde_json::from_value(res)?)
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

        let id = self.send_message(msg);

    }


}


