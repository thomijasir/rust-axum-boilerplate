use rand::{thread_rng, Rng};

pub struct Helper;

impl Helper {
  pub fn generate_id(len: Option<usize>) -> String {
    // With a specific length: Helper::generate_id(Some(15))
    // With the default length: Helper::generate_id(None)
    let length = len.unwrap_or(10);
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = thread_rng();

    (0..length)
      .map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
      })
      .collect()
  }
}
