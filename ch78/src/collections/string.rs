pub mod string {
  pub fn init_string() -> String {
      String::new()
  }

  pub fn vec_to_string(v: Vec<String>) -> String {
      let mut string = String::from("");
      for word in v {
          string.push_str(&word);
          string.push_str(" ");
      }
      string.to_string()
  }
}