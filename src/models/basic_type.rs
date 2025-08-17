use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum OneBotBool {
    True,
    False,
}

impl OneBotBool {
    pub const fn as_bool(&self) -> bool {
        match self {
            OneBotBool::True => true,
            OneBotBool::False => false,
        }
    }
}

impl From<OneBotBool> for bool {
    fn from(value: OneBotBool) -> Self {
        value.as_bool()
    }
}

impl From<bool> for OneBotBool {
    fn from(value: bool) -> Self {
        if value {
            OneBotBool::True
        } else {
            OneBotBool::False
        }
    }
}

impl Serialize for OneBotBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match self {
            OneBotBool::True => 1,
            OneBotBool::False => 0,
        })
    }
}

impl<'de> Deserialize<'de> for OneBotBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(OneBotBool::False),
            1 => Ok(OneBotBool::True),
            _ => Err(serde::de::Error::custom(format!(
                "invalid value: {value}, need 0 or 1",
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupRole {
    Owner,
    Admin,
    Member,
}
