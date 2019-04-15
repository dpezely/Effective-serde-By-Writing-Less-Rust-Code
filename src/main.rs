/// The example offered is a simple, realistic example of minimalist
/// JSON file where:
///
/// 1) Outer structure is an object (not an array)
/// 2) Top-level keys contain information (not name of structure)
/// 3) Inner values within array indicate mixed categories.
///
/// Write less Rust code overall simply by making use of various serde
/// attributes, the question-mark operator `?` and `ErrorKind` with
/// various impls of `From` and `Into` traits.

extern crate serde;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")] // See https://serde.rs/attributes.html
struct EnergyPreferenceHistory {
    energy_preferences: EnergyPreferences
}

#[derive(Serialize, Deserialize, Debug)]
struct EnergyPreferences (HashMap<Century, Vec<EnergySources>>);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
enum Century {
    #[serde(rename = "1800s")]
    NinteenthCentury,

    #[serde(rename = "1900s")]
    TwentiethCentury,

    #[serde(rename = "2000s")]
    TwentyfirstCentury
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]              // <-- Unflatten from compact JSON
enum EnergySources {
    Sustainable(Inexhaustible),
    Animal(Blubber),
    Vegetable(Crop),
    Mineral(Fossil),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Inexhaustible {
    Solar,
    Wind,
    // ...
}

/// Animal
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Blubber {
    Seal,
    Whale,
    // ...
}

/// Vegetable
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Crop {
    Peanut,
    Soy,
    // ...
}

/// Mineral
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Fossil {
    Kerosene,
    Petroleum,
    // ...
}

// Optionally, use Failure::Fail and annotate each variant with an
// error message, so you may eliminate `println!` statements within
// `from` methods below.  (Just keeping it simple, here)
#[derive(Debug)]
enum ErrorKind {
    BadJson,
    NoJson,
    NoFilePath,
    Unknown,
}

// TODO: as an exercise, comment-out all `impl From` and see how the
// compiler helpfully indicates *exactly* what needs to be written.
// Then, it's a matter of taste regarding how deep you go into
// addressing each particular error to your own ErrorKind.
// Lots to love about Rust!
impl From<serde_json::Error> for ErrorKind {
    fn from(err: serde_json::Error) -> ErrorKind {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => {
                println!("Serde JSON IO-error: {:?}", &err);
                ErrorKind::NoJson
            }
            Category::Syntax | Category::Data | Category::Eof => {
                println!("Serde JSON error: {:?} {:?}", err.classify(), &err);
                ErrorKind::BadJson
            }
        }
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(err: std::io::Error) -> ErrorKind {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File or directory path not found: {:?}", err);
                ErrorKind::NoFilePath
            }
            _ => {
                println!("IO Error: {:?}", err);
                ErrorKind::Unknown
            }
        }
    }
}

fn main() -> Result<(), ErrorKind> {
    let json_string = fs::read_to_string("energy.json")?;
    let sources: EnergyPreferenceHistory =
        serde_json::de::from_str(&json_string)?;
    println!("{:#?}", sources);
    Ok(())
}
