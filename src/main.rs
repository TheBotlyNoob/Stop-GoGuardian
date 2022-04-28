#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::sleep, time::Duration};

pub mod kill;

pub(crate) static PROCESS_NAME: &str = "Chromium.Goguardian.GGWindowsHost.exe";

fn main() {
  loop {
    kill::by_name(PROCESS_NAME);

    sleep(Duration::from_secs(10));
  }
}
