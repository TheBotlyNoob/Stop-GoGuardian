#![windows_subsystem = "windows"]

use std::{thread::sleep, time::Duration};
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

pub(crate) static PROCESS_NAME: &str = "Chromium.Goguardian.GGWindowsHost.exe";

fn main() {
  #[cfg(feature = "auth")]
  {
    use winrt_notification::{Duration as ToastDuration, Sound, Toast};

    let err_toast_fn = || {
      Toast::new(Toast::POWERSHELL_APP_ID)
        .title("You Haven't Paid")
        .sound(Some(Sound::Reminder))
        .duration(ToastDuration::Short)
        .show()
        .expect("unable to toast");

      std::process::exit(1);
    };

    if std::env::var("UserName").unwrap_or_else(|_| err_toast_fn())
      != std::include_str!("../.student-id").trim()
    {
      err_toast_fn();
    }
  }

  loop {
    for process in System::new_with_specifics(
      RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    )
    .processes_by_name(PROCESS_NAME)
    {
      process.kill();

      #[cfg(debug_assertions)]
      println!("Killed {}", PROCESS_NAME);
    }

    sleep(Duration::from_secs(6));
  }
}
