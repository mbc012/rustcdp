use std::collections::HashMap;
use crate::chrome::{Chrome, UserCallMessage};
use crate::error::{Result};


impl Chrome {
    pub fn disable_security(&mut self) {//} -> Result<()> {
        let msg = UserCallMessage::new("Security.disable");
        let id = self.send_message(msg);

    }


    pub fn enable_security(&mut self) {
        let msg = UserCallMessage::new("Security.enable");
        let id = self.send_message(msg);

    }

    pub fn set_ignore_certificate_errors(&mut self, ignore: bool) {
        let msg = UserCallMessage::new("Security.setIgnoreCertificateErrors")
            .set_params({
                let mut hm = HashMap::new();
                hm.insert("ignore".into(), ignore.into());
                hm
            });
        let id = self.send_message(msg);

    }
}


