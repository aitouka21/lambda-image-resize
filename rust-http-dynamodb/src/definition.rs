use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumString;
use ulid::Ulid;

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Ordering {
    Asc,
    Desc,
}

impl From<Ordering> for bool {
    fn from(val: Ordering) -> Self {
        match val {
            Ordering::Asc => true,
            Ordering::Desc => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseOrderingError;

impl Default for Ordering {
    fn default() -> Self {
        Ordering::Desc
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ReportReason {
    Spam,
    Violence,
    On9,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct ReportPk;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(transparent)]
struct ReportSk(Ulid);

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct ReportKey {
    pk: ReportPk,
    sk: ReportSk,
}

pub type ReportCursor = ReportKey;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCursorError;

impl FromStr for ReportCursor {
    type Err = ParseCursorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Ulid>() {
            Ok(id) => Ok(ReportCursor {
                pk: ReportPk,
                sk: ReportSk(id),
            }),
            Err(_) => Err(ParseCursorError),
        }
    }
}

impl ReportCursor {
    pub fn to_token(self) -> String {
        self.sk.0.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportItem {}
