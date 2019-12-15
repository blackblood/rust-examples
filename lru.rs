use std::collections::HashMap;
use std::collections::VecDeque;

struct LRUCache {
  map: HashMap<String, String>,
  que: VecDeque<String>,
  max_length: u8
}

impl LRUCache {
  fn new() -> LRUCache {
    LRUCache { map: HashMap::new(), que: VecDeque::new(), max_length: 5 }
  }

  fn put(&mut self, key: String, value: String) {
    self.map.insert(key, value.clone());
    if self.que.len() > 0 {
      let mut i = 0;
      let mut del_index = 0;
      for el in self.que.iter() {
        if value.as_str() == el {
          del_index = i;
        }
        i += 1;
      }
      self.que.remove(del_index);
      self.que.push_front(value.clone());
    }
    if (self.que.len() as u8) > self.max_length {
      if let Some(last_el) = self.que.pop_back() {
        self.map.remove(&last_el);
      }
    }
  }

  fn get(&self, key: String) -> Option<String> {
    if let Some(v) = self.map.get(&key) {
      Some(v.to_string())
    } else {
      None
    }
  }
}

fn main() {
  let mut cache = LRUCache::new();
  cache.put("akshay".to_string(), "27".to_string());
  cache.put("aditya".to_string(), "30".to_string());
  cache.put("kasturi".to_string(), "61".to_string());
  cache.put("vijay".to_string(), "67".to_string());

  if let Some(cache_value) = cache.get("sandeep".to_string()) {
    println!("{}", cache_value);
  } else {
    println!("Not found in cache");
  }
}