use super::*;

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
  fs::create_dir_all(&dst)?;
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let ty = entry.file_type()?;
    if ty.is_dir() {
      copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
    }
  }
  Ok(())
}

pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let path = entry.path();

    if entry.file_type()?.is_dir() {
      remove_dir_contents(&path)?;
      fs::remove_dir(path)?;
    } else {
      fs::remove_file(path)?;
    }
  }
  Ok(())
}