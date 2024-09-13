use std::collections::HashMap;
use serde_json::Value;
use crate::chrome::{Chrome, UserCallMessage};
use crate::chrome::domain::browser::types::{BrowserContextID, BrowserVersion};
use crate::error::{Result, Error};

// todo: Consideration: Should I add a Result<()> for void fn's. I suspect may need a lot of testing to impl.


impl Chrome {

    /// Allows a site to use privacy sandbox features that require enrollment without the site actually being enrolled. Only supported on page targets.
    /// url - string
    pub fn add_privacy_sandbox_enrollment_override<U: Into<Value>>(&mut self, url: U) {
        let msg = UserCallMessage::new("Browser.addPrivacySandboxEnrollmentOverride")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("url".into(), url.into());
                hm
            });

        let idx = self.send_message(msg);
    }


    /// Close browser gracefully.
    pub fn close(&mut self) {
        let msg = UserCallMessage::new("Browser.close");
        let idx = self.send_message(msg);
    }

    /// Returns version information.
    pub fn get_version(&mut self) -> Result<BrowserVersion> {
        let msg = UserCallMessage::new("Browser.getVersion");
        let id = self.send_message(msg);
        let val = self.wait_ucr(id)?;
        Ok(serde_json::from_value(val)?)
    }

    /// Returns version information.
    pub fn reset_permissions(&mut self, browser_context_id: Option<BrowserContextID>) {
        let msg = UserCallMessage::new("Browser.resetPermissions");
        let id = self.send_message(msg);
    }


}
