use toml;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub mutations_per_generation: usize,
    pub initial_program_length: usize,
    pub max_runtime: usize,
    pub population_size: usize,
    pub targets: Vec<Vec<u32>>,
    pub inputs: Vec<Vec<u32>>,
    legacy: Option<bool>,
    free_mut: Option<bool>,
}

impl Configuration {
    // Check if the configuration indicates legacy mode; it is false by default.
    pub fn is_legacy(&self) -> bool {
        if let Some(l) = self.legacy {
            l 
        } else { false }
    }

    // Check if the configuration indicates free_mut mode; it is false by default.
    pub fn is_free_mut(&self) -> bool {
        if let Some(l) = self.free_mut {
            l 
        } else { false }
    }
}

use std::path::Path;
pub fn read_config(filename: &Path) -> Configuration {
    use std::io::prelude::*;
    use std::fs::File;

    let mut config = String::new();
    let mut f = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {panic!(format!("Failed to open file {}: {}", filename.to_string_lossy(), e));}
    };

    f.read_to_string(&mut config).expect("Could not read from file.");

    let config: Configuration = match toml::from_str(&config) {
        Ok(c) => c,
        Err(e) => {panic!(format!("Failed to parse configuration file: {}", e))}
    };

    // Verify that there are the right number of inputs and outputs
    if config.inputs.len() != config.targets.len() {
        panic!(format!("Invalid configuration; number of inputs and outputs should match, but there are {} inputs and {} outputs.",
            config.inputs.len(), config.targets.len()))
    }

    config
}