#[derive(Clone, Default, Debug)]
pub struct Mount {
  pub device: String,
  pub mount_point: String,
  pub file_system_type: String,
  pub options: Vec<String>,
}

fn main() {
  let mount3 = Mount {
    device: "C:\\".to_string(),
    mount_point: "/mnt/c".to_string(),
    file_system_type: "9p".to_string(),
    options: [
      "rw",
      "dirsync",
      "noatime",
      "aname=drvfs;path=C:\\;uid=1000;gid=1000;symlinkroot=/mnt/",
      "mmap",
      "access=client",
      "msize=65536",
      "trans=fd",
      "rfd=8",
      "wfd=8",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect(),
  };
  Ok(());
}
