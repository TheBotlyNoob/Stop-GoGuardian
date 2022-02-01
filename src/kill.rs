use std::ptr;
use windows::Win32::{
  Foundation::CloseHandle,
  System::{
    Diagnostics::ToolHelp::{
      CreateToolhelp32Snapshot, Process32First, PROCESSENTRY32, TH32CS_SNAPALL,
    },
    Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE},
  },
};

// https://stackoverflow.com/a/7956651 in Rust
pub fn by_name(filename: impl AsRef<str>) {
  let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPALL, 0) };
  let mut process_entry = PROCESSENTRY32::default();
  let mut res = unsafe { Process32First(snapshot, &mut process_entry).as_bool() };

  println!(
    "l: {:#?}",
    String::from_utf8(process_entry.szExeFile.map(|char| char.0).to_vec()).unwrap()
  );

  while res {
    if std::str::from_utf8(&process_entry.szExeFile.map(|char| char.0)) == Ok(filename.as_ref()) {
      let process_handle =
        unsafe { OpenProcess(PROCESS_TERMINATE, None, process_entry.th32ProcessID) };

      if !process_handle.is_invalid() {
        unsafe { TerminateProcess(process_handle, 9) };
        unsafe { CloseHandle(process_handle) };
      }
    }

    res = unsafe { Process32First(snapshot, ptr::addr_of_mut!(process_entry)).as_bool() };
  }
  unsafe { CloseHandle(snapshot) };
}
