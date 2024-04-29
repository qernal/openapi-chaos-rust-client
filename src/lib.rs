#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

extern crate serde_qs;

fn query_to_pairs(query: &impl serde::Serialize) -> Vec<(String, String)> {
    let query = serde_qs::to_string(&query).unwrap();
    query
        .split('&')
        .map(|v| {
            let (lhs, rhs) = v.split_once('=').unwrap();
            let rhs = urlencoding::decode(rhs).unwrap().into();
            (lhs.into(), rhs)
        })
        .collect()
}

pub mod apis;
pub mod models;
