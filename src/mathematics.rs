pub fn vector_norm(vec_a: &[f32]) -> f32 {
  let mut norm = 0 as f32;
  for i in 0..vec_a.len() {
    norm += vec_a[i] * vec_a[i];
  }
  norm.sqrt()
}
  
pub fn vector_diff(vec_a: &[f32], vec_b: &[f32]) -> f32 {
  let mut diff = Vec::new() as Vec<f32>;
  assert!(vec_a.len() == vec_b.len(), "Vector lenghts must be equal for arguments in vector_diff");
  for i in 0..vec_a.len() {
    diff.push(vec_a[i] - vec_b[i]);
  }
  vector_norm(&diff)
}
  