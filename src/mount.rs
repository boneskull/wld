
use std::string::String;
use std::vec::Vec;

#[derive(Clone, Default, Debug)]
pub struct Mount {
  pub device: String,
  pub mount_point: String,
  pub file_system_type: String,
  pub options: Vec<String>,
}

/// Implements `Display` for `Mount` to simulate behavior of Unix mount command.
///
/// # Examples
/// ```
/// # use wld::Mount;
/// # use std::string::String;
/// let mount = Mount {
/// 	device: String::from("/dev/sda1"),
/// 	mount_point: String::from("/mnt/disk"),
/// 	file_system_type: String::from("ext4"),
/// 	options: vec![String::from("ro"), String::from("nosuid")]
/// };
/// assert!(mount.to_string() == "/dev/sda1 on /mnt/disk type ext4 (ro,nosuid)");
/// ```
impl std::fmt::Display for Mount {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} on {} type {} ({})",
      self.device,
      self.mount_point,
      self.file_system_type,
      self.options.join(",")
    )
  }
}

#[cfg(test)]
macro_rules! mount {
  ($device: expr, $mount_point: expr, $fs_type: expr, $($opt:expr),*) => {{
    let mut options: Vec<String> = Vec::new();
    $(options.push($opt.to_string());)*
    Mount {
      device: $device.to_string(),
      mount_point: $mount_point.to_string(),
      file_system_type: $fs_type.to_string(),
      options: options
    }
  }};
}
