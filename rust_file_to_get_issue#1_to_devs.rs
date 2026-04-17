fn main(){

  println("Hello World, this file exists soly so that this repo has Rust in it so it finds its way to Rust devs who may help.");
  
  
}


// Random Gemini code thats technicaly Rust

use std::collections::HashMap;

// A custom error type for our system
#[derive(Debug)]
pub enum PowerError {
    CriticalFailure(String),
    InsufficientFlux,
}

// A trait to define how components consume energy
pub trait EnergyConsumer {
    fn consume(&self, amount: u32) -> Result<(), PowerError>;
    fn status(&self) -> String;
}

struct Reactor {
    id: u32,
    output_mw: u32,
    active: bool,
}

impl EnergyConsumer for Reactor {
    fn consume(&self, amount: u32) -> Result<(), PowerError> {
        if amount > self.output_mw {
            return Err(PowerError::InsufficientFlux);
        }
        println!("Reactor {} processing {}MW request...", self.id, amount);
        Ok(())
    }

    fn status(&self) -> String {
        format!("Reactor {}: [Active={}]", self.id, self.active)
    }
}

pub fn a() {
    let mut grid: HashMap<String, Reactor> = HashMap::new();

    // Populate the grid
    grid.insert(
        "Alpha-1".to_string(),
        Reactor { id: 101, output_mw: 500, active: true },
    );

    // Using pattern matching to handle results
    let request_load = 250;
    
    if let Some(core) = grid.get("Alpha-1") {
        match core.consume(request_load) {
            Ok(_) => println!("Grid stable. Output: {}", core.status()),
            Err(e) => eprintln!("System alert: {:?}", e),
        }
    } else {
        println!("Error: Primary core not found in registry.");
    }

    // A simple closure to demonstrate functional style
    let active_cores: Vec<&Reactor> = grid.values().filter(|r| r.active).collect();
    println!("Active cores online: {}", active_cores.len());
}

// Random Gemini code thats technicaly Rust

use std::collections::HashMap;

// A custom error type for our system
#[derive(Debug)]
pub enum PowerError {
    CriticalFailure(String),
    InsufficientFlux,
}

// A trait to define how components consume energy
pub trait EnergyConsumer {
    fn consume(&self, amount: u32) -> Result<(), PowerError>;
    fn status(&self) -> String;
}

struct Reactor {
    id: u32,
    output_mw: u32,
    active: bool,
}

impl EnergyConsumer for Reactor {
    fn consume(&self, amount: u32) -> Result<(), PowerError> {
        if amount > self.output_mw {
            return Err(PowerError::InsufficientFlux);
        }
        println!("Reactor {} processing {}MW request...", self.id, amount);
        Ok(())
    }

    fn status(&self) -> String {
        format!("Reactor {}: [Active={}]", self.id, self.active)
    }
}

pub fn b() {
    let mut grid: HashMap<String, Reactor> = HashMap::new();

    // Populate the grid
    grid.insert(
        "Alpha-1".to_string(),
        Reactor { id: 101, output_mw: 500, active: true },
    );

    // Using pattern matching to handle results
    let request_load = 250;
    
    if let Some(core) = grid.get("Alpha-1") {
        match core.consume(request_load) {
            Ok(_) => println!("Grid stable. Output: {}", core.status()),
            Err(e) => eprintln!("System alert: {:?}", e),
        }
    } else {
        println!("Error: Primary core not found in registry.");
    }

    // A simple closure to demonstrate functional style
    let active_cores: Vec<&Reactor> = grid.values().filter(|r| r.active).collect();
    println!("Active cores online: {}", active_cores.len());
}

// Random Gemini code thats technicaly Rust

use std::collections::HashMap;

// A custom error type for our system
#[derive(Debug)]
pub enum PowerError {
    CriticalFailure(String),
    InsufficientFlux,
}

// A trait to define how components consume energy
pub trait EnergyConsumer {
    fn consume(&self, amount: u32) -> Result<(), PowerError>;
    fn status(&self) -> String;
}

struct Reactor {
    id: u32,
    output_mw: u32,
    active: bool,
}

impl EnergyConsumer for Reactor {
    fn consume(&self, amount: u32) -> Result<(), PowerError> {
        if amount > self.output_mw {
            return Err(PowerError::InsufficientFlux);
        }
        println!("Reactor {} processing {}MW request...", self.id, amount);
        Ok(())
    }

    fn status(&self) -> String {
        format!("Reactor {}: [Active={}]", self.id, self.active)
    }
}

pub fn c() {
    let mut grid: HashMap<String, Reactor> = HashMap::new();

    // Populate the grid
    grid.insert(
        "Alpha-1".to_string(),
        Reactor { id: 101, output_mw: 500, active: true },
    );

    // Using pattern matching to handle results
    let request_load = 250;
    
    if let Some(core) = grid.get("Alpha-1") {
        match core.consume(request_load) {
            Ok(_) => println!("Grid stable. Output: {}", core.status()),
            Err(e) => eprintln!("System alert: {:?}", e),
        }
    } else {
        println!("Error: Primary core not found in registry.");
    }

    // A simple closure to demonstrate functional style
    let active_cores: Vec<&Reactor> = grid.values().filter(|r| r.active).collect();
    println!("Active cores online: {}", active_cores.len());
}

// Random Gemini code thats technicaly Rust

use std::collections::HashMap;

// A custom error type for our system
#[derive(Debug)]
pub enum PowerError {
    CriticalFailure(String),
    InsufficientFlux,
}

// A trait to define how components consume energy
pub trait EnergyConsumer {
    fn consume(&self, amount: u32) -> Result<(), PowerError>;
    fn status(&self) -> String;
}

struct Reactor {
    id: u32,
    output_mw: u32,
    active: bool,
}

impl EnergyConsumer for Reactor {
    fn consume(&self, amount: u32) -> Result<(), PowerError> {
        if amount > self.output_mw {
            return Err(PowerError::InsufficientFlux);
        }
        println!("Reactor {} processing {}MW request...", self.id, amount);
        Ok(())
    }

    fn status(&self) -> String {
        format!("Reactor {}: [Active={}]", self.id, self.active)
    }
}

pub fn d() {
    let mut grid: HashMap<String, Reactor> = HashMap::new();

    // Populate the grid
    grid.insert(
        "Alpha-1".to_string(),
        Reactor { id: 101, output_mw: 500, active: true },
    );

    // Using pattern matching to handle results
    let request_load = 250;
    
    if let Some(core) = grid.get("Alpha-1") {
        match core.consume(request_load) {
            Ok(_) => println!("Grid stable. Output: {}", core.status()),
            Err(e) => eprintln!("System alert: {:?}", e),
        }
    } else {
        println!("Error: Primary core not found in registry.");
    }

    // A simple closure to demonstrate functional style
    let active_cores: Vec<&Reactor> = grid.values().filter(|r| r.active).collect();
    println!("Active cores online: {}", active_cores.len());
}
