// Evita que se abra una consola negra en Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    biblioteca_virtual_lib::run()
}
