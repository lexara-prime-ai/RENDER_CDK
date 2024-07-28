#![allow(non_snake_case)]

use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

pub fn GENERATE_RANDOM_STRING(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn GENERATE_UNIQUE_NAME() -> String {
    let adjectives = [
        "agile",
        "brisk",
        "cyber",
        "dynamic",
        "efficient",
        "fluent",
        "geared",
        "hyper",
        "instant",
        "keen",
        "luminous",
        "mighty",
        "nimble",
        "optimum",
        "proactive",
        "quick",
        "rapid",
        "swift",
        "tech",
        "ultra",
        "vivid",
        "witty",
        "xenial",
        "youthful",
        "zealous",
    ];

    let nouns = [
        "Chronos", "Hyperion", "Aether", "Quanta", "Nebula", "Vortex", "Zenith", "Pulsar", "Nova",
        "Orion", "Stratus", "Nimbus", "Cosmos", "Eclipse", "Helix", "Photon", "Spectra", "Aurora",
        "Cypher", "Terra", "Quasar", "Solstice", "Astro", "Comet", "Lunar", "Matrix", "Atlas",
        "Inferno", "Blaze", "Cyclone", "Titan", "Galaxy", "Horizon", "Mistral", "Phoenix",
        "Radiance", "Meteor", "Plasma", "Inferno", "Echo", "Vanguard", "Stellar", "Fusion", "Halo",
        "Spectrum", "Zephyr", "Draco", "Equinox", "Falcon",
    ];

    let mut rng = thread_rng();
    let adjective = adjectives.choose(&mut rng).unwrap();
    let noun = nouns.choose(&mut rng).unwrap().to_lowercase();

    format!("{}{}", adjective, noun)
}
