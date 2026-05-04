use derive_getters::Getters;
use regex::Regex;
use semver_common::Alert;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod regex_ser {
    use serde::de;

    use super::*;

    pub fn serialize<S>(ext: &Regex, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&ext.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Regex, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let r = match Regex::new(&s) {
            Ok(v) => v,
            Err(e) => return Err(de::Error::custom(format!("Regex::Error: {}", e))),
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Getters)]
pub struct Rule {
    #[serde(with = "regex_ser")]
    pattern: Regex,

    message: String,
}

impl Rule {
    pub fn check(&self, commit: &str) -> Result<(), Alert> {
        if self.pattern.find(commit).is_none() {
            return Err(Alert::from(&self.message));
        }
        Ok(())
    }
}
