use std::collections::HashMap;
use serde_json::Value;
use crate::chrome::{Chrome, UserCallMessage};
use crate::chrome::domain::browser::types::{BrowserContextID, BrowserVersion};
use crate::error::{Result, Error};


impl Chrome {

    /// Allows a site to use privacy sandbox features that require enrollment without the site actually being enrolled. Only supported on page targets.
    /// url - string
    pub fn add_privacy_sandbox_enrollment_override<U: Into<Value>>(&mut self, url: U) -> Result<()>{
        let msg = UserCallMessage::new("Browser.addPrivacySandboxEnrollmentOverride")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("url".into(), url.into());
                hm
            });

        let id = self.send_message(msg);
        let _ = self.wait_ucr(id)?;
        Ok(())
    }


    /// Close browser gracefully.
    pub fn close(&mut self) -> Result<()> {
        let msg = UserCallMessage::new("Browser.close");
        let id = self.send_message(msg);
        let _ = self.wait_ucr(id)?;
        Ok(())
    }

    /// Returns version information.
    pub fn get_version(&mut self) -> Result<BrowserVersion> {
        let msg = UserCallMessage::new("Browser.getVersion");
        let id = self.send_message(msg);
        let val = self.wait_ucr(id)?;
        Ok(serde_json::from_value(val)?)
    }

    /// Returns version information.
    pub fn reset_permissions(&mut self, browser_context_id: Option<BrowserContextID>) -> Result<()> {
        let msg = UserCallMessage::new("Browser.resetPermissions");
        let id = self.send_message(msg);
        let _ = self.wait_ucr(id)?;
        Ok(())
    }


}
