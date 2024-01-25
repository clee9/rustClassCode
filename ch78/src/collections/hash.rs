pub mod hash {
  use std::collections::HashMap;

  pub fn init_hash() -> HashMap<String, u32> {
      HashMap::new()
  }
  
  pub fn get_hash<'a, 'b>(hash: &'a HashMap<String, u32>, word: &'b String) -> Option<&'a u32> {
      hash.get(word)
  }

  pub fn add_to_hash(hash: & mut HashMap<String, u32>, input: &String, count: u32) {
      hash.insert(input.clone(), count);
  }
}