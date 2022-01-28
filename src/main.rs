#![windows_subsystem = "windows"]

use std::{thread::sleep, time::Duration};
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

pub(crate) static PROCESS_NAME: &str = "Chromium.Goguardian.GGWindowsHost.exe";

fn main() {
    loop {
        for process in System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
        )
        .processes_by_name(PROCESS_NAME)
        {
            process.kill();
        }

        sleep(Duration::from_secs(6));
    }
}
