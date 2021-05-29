use rand::Rng;
use rand::distributions::Standard;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
  let mut rng = rand::thread_rng();
  rng.sample_iter(&Standard).take(n).collect()
}
pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] <= pair[1])
}
pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] >= pair[1])
}