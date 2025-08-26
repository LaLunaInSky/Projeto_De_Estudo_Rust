use std::{
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal
};

pub fn rodar_final_do_exercício() {
    sleep(Duration::from_millis(1500));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(2000));

    limpar_terminal();
}