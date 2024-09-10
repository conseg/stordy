



pub struct BTree {
  db: sled::Db,
}

impl BTree {
  pub fn new(path: &str) -> BTree {
    let db: sled::Db = sled::open(path).unwrap();
    BTree {
      db,
    }
  }

  pub fn insert(&self, key: String, value: Vec<u8>) {
    self.db.insert(key, value).unwrap();
  }

  pub fn get(&self, key: String) -> Option<Vec<u8>> {
    match self.db.get(key).unwrap() {
      Some(value) => Some(value.to_vec()),
      None => None,
    }
  }

  /*
    BA TA1   TA2
    BB    TB2
   */

  pub fn get_last(&self) -> Option<(String, Vec<u8>)> {
    let last = self.db.iter().last();
    if !last.is_some() {
      return None;
    }

    let last = last.unwrap();
    match last {
      Ok((key, value)) => Some((String::from_utf8(key.to_vec()).unwrap(), value.to_vec())),
      _ => None,
    }

  }

  pub fn len(&self) -> usize {
    self.db.len()
  }

  pub fn iter(&self) -> sled::Iter {
    self.db.iter()
  }


}