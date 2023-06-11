use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{ self, BufRead };

pub struct DotEnv {
  filename: String,
  data: HashMap<String, String>
}

impl DotEnv {
  fn new(filename: &str) -> DotEnv {
    DotEnv {
      filename: filename.to_string(),
      data: HashMap::new()
    }
  }

  // Load

  fn load(
    &mut self
  ) -> Result<(), String> {
    let lines = match Self::read_lines(&self.filename) {
      Ok(lines) => lines,
      Err(_) => return Err(format!("Failed to read file {}", self.filename))
    };
    for l in lines {
      if let Err(_) = l {
        return Err(format!("Failed to read line in file {}", self.filename));
      }
      let l = l.unwrap();
      if l.starts_with("#") || l.len() == 0 {
        continue;
      }
      if l.matches("=").count() != 1 {
        return Err(format!("Invalid line in file {}:\n{}", self.filename, l));
      }
      let parts = l.split("=").collect::<Vec<&str>>();
      match parts.len() {
        2 => {
          self.data.insert(
            parts[0].trim().to_string(),
            parts[1].trim().to_string()
          );
        },
        _ => return Err(format!("Invalid line in file {}", self.filename))
      }
    }
    Ok(())
  }

  fn from_filename(filename: &str) -> Result<DotEnv, String> {
    let mut dotenv = DotEnv::new(filename);
    match dotenv.load() {
      Ok(_) => Ok(dotenv),
      Err(e) => Err(e)
    }
  }

  fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
  }

  // Getters

  pub fn var(&self, key: &str) -> Option<&String> {
    self.data.get(key)
  }
}

pub fn from_filename(filename: &str) -> Result<DotEnv, String> {
  DotEnv::from_filename(filename)
}