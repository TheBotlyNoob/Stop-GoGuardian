#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::sleep, time::Duration};

pub mod kill;

pub(crate) static PROCESS_NAME: &str = "Chromium.Goguardian.GGWindowsHost.exe";

fn main() {
  #[cfg(feature = "auth")]
  {
    use winrt_notification::{Duration as ToastDuration, Sound, Toast};

    let err_toast_fn = |message| {
      Toast::new(Toast::POWERSHELL_APP_ID)
        .title(message)
        .sound(Some(Sound::Reminder))
        .duration(ToastDuration::Short)
        .show()
        .expect("unable to toast");

      std::process::exit(1);
    };

    if std::env::var("UserName").unwrap_or_else(|_| err_toast_fn("Error Getting Username"))
      != std::include_str!("../.student-id").trim()
    {
      err_toast_fn("You Haven't Paid");
    }
  }

  loop {
    kill::by_name(PROCESS_NAME);
    println!("Killed {}", PROCESS_NAME);

    sleep(Duration::from_secs(6));
  }
}
