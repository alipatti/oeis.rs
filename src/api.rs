use itertools::Itertools;
use serde::{de::Error, Deserialize};

use crate::{Sequence, OEIS_URL};
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct Response {
    results: Vec<Sequence>,

    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}

pub(crate) fn search(
    query: &[i32],
) -> Result<Vec<Sequence>, Box<dyn std::error::Error>> {
    let url =
        format!("{OEIS_URL}/search?q={}&fmt=json", query.iter().join(","));
    let Response { results, _extra } = reqwest::blocking::get(url)?.json()?;

    Ok(results)
}

pub(crate) fn deserialize_sequence<'de, D>(
    deserializer: D,
) -> Result<Vec<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .split(',')
        .map(|x| x.parse().map_err(D::Error::custom))
        .collect()
}

pub(crate) fn open_sequence(id: usize) {
    let url = format!("{}/A{:0>6}", OEIS_URL, id);
    open::that(url).expect("Unable to open sequence on OEIS.");
}
