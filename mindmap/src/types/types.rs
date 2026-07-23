

pub enum VectorTypes {
    dense128([f32; 128]),
    dense256([f32; 256]),
    dense768([f32; 768]),
    sparse(HashMap<usize, f32>),
    custom(Vec<f32>),
}