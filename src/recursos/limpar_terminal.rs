use std::{
    process::Command
};

pub fn limpar_terminal() {
    Command::new("clear").status().unwrap();
}