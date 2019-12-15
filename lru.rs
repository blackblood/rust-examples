use std::collections::HashMap;
use std::collections::VecDeque;

struct LRUCache<T> {
  map: HashMap<String, T>,
  que: VecDeque<String>,
  max_length: u8
}

impl<T> LRUCache<T> {
  fn new() -> LRUCache<T> {
    LRUCache { map: HashMap::new(), que: VecDeque::new(), max_length: 5 }
  }

  fn put(&mut self, key: String, value: T) {
    self.map.insert(key.clone(), value);
    if self.que.len() > 0 {
      let mut i = 0;
      let mut del_index = 0;
      for el in self.que.iter() {
        if key.as_str() == el {
          del_index = i;
        }
        i += 1;
      }
      self.que.remove(del_index);
      self.que.push_front(key.clone());
    }
    if (self.que.len() as u8) > self.max_length {
      if let Some(last_el) = self.que.pop_back() {
        self.map.remove(&last_el);
      }
    }
  }

  fn get(&self, key: String) -> Option<&T> {
    self.map.get(&key)
  }
}

fn main() {
  let mut cache = LRUCache::new();
  cache.put("akshay".to_string(), "27".to_string());
  cache.put("aditya".to_string(), "30".to_string());
  cache.put("kasturi".to_string(), "61".to_string());
  cache.put("vijay".to_string(), "67".to_string());

  if let Some(cache_value) = cache.get("kasturi".to_string()) {
    println!("{:?}", cache_value);
  } else {
    println!("Not found in cache");
  }
}