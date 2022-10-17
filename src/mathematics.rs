use std::convert::TryInto;
use num_traits::real::Real;

const LANES: usize = 16;

pub fn euclidean_magnitude<T>(vec_a: &[T]) -> T 
  where T: std::ops::Mul + std::ops::Mul<Output = T> + From<f32> + std::ops::AddAssign + num_traits::Float {
  let mut norm: T = 0.0f32.try_into().unwrap();
  for i in 0..vec_a.len() {
    norm += vec_a[i] * vec_a[i];
  }
  norm.sqrt()
}

pub fn vector_euclidean_distance<T>(vec_a: &[T], vec_b: &[T]) -> T 
  where T: std::ops::Sub + std::ops::Sub<Output = T> + std::ops::Mul 
  + std::ops::Mul<Output = T> + From<f32> + std::ops::AddAssign + num_traits::Float {
  let mut diff = Vec::new() as Vec<T>;
  assert!(vec_a.len() == vec_b.len(), "Vector lenghts must be equal for arguments in vector_euclidean_distance");
  for i in 0..vec_a.len() {
    diff.push(vec_a[i] - vec_b[i]);
  }
  euclidean_magnitude(&diff)
}

pub fn minkowski_magnitude<T>(vec_a: &[T], p: T) -> T 
  where T: std::convert::From<f32> + num_traits::Float + std::ops::AddAssign {
  let mut norm: T = 0.0f32.try_into().unwrap();
  let numerator: T = 1.0f32.try_into().unwrap();
  for i in 0..vec_a.len() {
    norm += vec_a[i].powf(p);
  }
  norm.powf(numerator/p)
}
  
pub fn vector_minkowski_distance<T>(vec_a: &[T], vec_b: &[T], p: T) -> T 
  where T: std::ops::Sub + std::convert::From<f32> + num_traits::Float + std::ops::AddAssign {
  let mut diff = Vec::new() as Vec<T>;
  assert!(vec_a.len() == vec_b.len(), "Vector lenghts must be equal for arguments in vector_minkowski_distance");
  for i in 0..vec_a.len() {
    diff.push(vec_a[i] - vec_b[i]);
  }
  minkowski_magnitude(&diff, p)
}

pub fn vector_cosine_distance<T>(vec_a: &[T], vec_b: &[T]) -> T 
  where T: std::ops::Mul + std::ops::Mul<Output = T> + From<f32> + std::ops::AddAssign + num_traits::Float {
  assert!(vec_a.len() == vec_b.len(), "Vector lenghts must be equal for arguments in vector_cosine_distance");
  let mut numerator: T = 0.0f32.try_into().unwrap();
  for i in 0..vec_a.len() {
    numerator+=vec_a[i] * vec_b[i];
  }
  numerator/euclidean_magnitude(vec_a)/euclidean_magnitude(vec_b)
}


// pub fn slices_vecs_to_vect_of_slices<T>(input: &[Vec<T>]) -> Vec<&[T]> {
//   input.iter().map(Vec::as_slice).collect::<Vec<&[T]>>()
// }
// slices_vecs_to_vect_of_slices::<T>(input.as_slice()).as_slice(); -> &[&[T]]

pub fn transpose_vec2d<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
  assert!(!input.is_empty());
  let dlen = input[0].len();
  let mut diter: Vec<_> = input.into_iter().map(|x| x.into_iter()).collect();
  (0..dlen).map(|_| {
    diter.iter_mut().map(|x2| x2.next().unwrap()).collect::<Vec<T>>()
  }).collect()
}

pub fn nonsimd_sum<T>(values: &[T]) -> T 
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign + num_traits::Zero + From<f32> {
  let chunks = values.chunks_exact(LANES);
  let remainder = chunks.remainder();
  let sum: [T; 16] = chunks.fold([0.0f32.try_into().unwrap(); LANES], |mut acc, chunk| {
    let chunk: [T; LANES] = chunk.try_into().unwrap();
    for i in 0..LANES {
      acc[i] += chunk[i];
    }
    acc
  });
  let remainder: T = remainder.iter().copied().sum();
  let mut reduced : T = 0.0f32.try_into().unwrap();
  for i in 0..LANES {
    reduced += sum[i];
  }
  reduced + remainder
}
pub fn vec1d_sum<T>(input: &Vec<T>) -> T 
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign + num_traits::Zero + From<f32> {
    nonsimd_sum::<T>(input.as_slice())
}
pub fn vec2d_axis_sum<T>(input: &Vec<Vec<T>>, dimension: usize) -> Vec<T>
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign + num_traits::Zero + From<f32> {
  assert!(dimension==0 || dimension==1);
  if dimension==0 {
    transpose_vec2d::<T>(input.clone()).iter().map(|v| nonsimd_sum(v.as_slice())).collect::<Vec<T>>()
  } else {
    input.iter().map(|v| nonsimd_sum(v.as_slice())).collect::<Vec<T>>()
  }
}
pub fn vec2d_axis_average<T>(input: &Vec<Vec<T>>, dimension: usize) -> Vec<T>
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign + num_traits::Zero + From<f32> + From<i16> {
    assert!(dimension==0 || dimension==1);
    if dimension==0 {
      let n: T = (input.len() as i16).try_into().unwrap();
      vec2d_axis_sum::<T>(input, dimension).iter().map(|&x| x / n).collect()
    } else {
      let n: T = (input.get(0).as_ref().unwrap().len() as i16).try_into().unwrap();
      vec2d_axis_sum::<T>(input, dimension).iter().map(|&x| x / n).collect()
    }
}
pub fn vec1d_normalize_mu1<T>(input: &Vec<T>) -> Vec<T> 
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign + num_traits::Zero + From<f32> {
    let magnitude: T = vec1d_sum::<T>(&input);
    input.iter().map(|x| *x/magnitude).collect::<Vec<T>>()
}
pub fn vec1d_normalize_mu2<T>(input: &Vec<T>) -> Vec<T> 
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign 
    + num_traits::Zero + From<f32> + num_traits::Float {
    let magnitude: T = vec1d_sum::<T>(&input.iter().map(|&x|x.abs()).collect::<Vec<T>>());
    input.iter().map(|x| *x/magnitude).collect::<Vec<T>>()
}
pub fn vec1d_binary_entropy<T>(input: &Vec<T>) -> T 
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign + num_traits::Zero + From<f32>  + Real {
    <f32 as TryInto<T>>::try_into(1.0f32).unwrap() * nonsimd_sum::<T>(input.iter().map(|x| (*x)*x.log2()).collect::<Vec<T>>().as_slice())
}
pub fn vec1d_normalize_binary_entropy<T>(input: &Vec<T>) -> T 
  where T: 'static + num_traits::Num + Copy + std::iter::Sum + std::ops::AddAssign + num_traits::Zero + From<f32>  + Real +  num_traits::Float {
    vec1d_binary_entropy::<T>(&vec1d_normalize_mu2::<T>(input))
}