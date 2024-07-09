#![allow(unused)]
use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self};

#[allow(non_snake_case)]
pub struct State {
    CLIENT: reqwest::Client,
}


