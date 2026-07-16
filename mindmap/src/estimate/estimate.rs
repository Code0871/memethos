// function for computing memory usage

pub struct Dimension(pub usize);
pub struct DotCount(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    F32,
    F64,
    F16,
    I16,
    I8,
}

impl DataType {
    pub fn bytes(&self) -> usize {
        match self {
            DataType::F32 => 4,
            DataType::F64 => 8,
            DataType::F16 => 2,
            DataType::I8 => 1,
            DataType::I16 => 2,
        }
    }
}

pub struct EstimateInput {
    pub dimension: Dimension,       // vector size for example 768
    pub dot_count: DotCount,        // number of dots iin vestor space
    pub dtype: DataType,            // data type elements in vector
    pub vectors_per_dot: usize,     // how many vectors are used for dot
}

pub struct MemoryEstimate {
    pub total_bytes: usize,
    pub total_mb: f64,
}

pub fn estimate(input: EstimateInput) -> MemoryEstimate {
    // вычисляем память в байтах
    let total_bytes = input.dimension.0 
        * input.dtype.bytes() 
        * input.dot_count.0 
        * input.vectors_per_dot;
    
    let total_mb = total_bytes as f64 / (1024.0 * 1024.0);
    
    println!("📊 Memory usage ONLY FOR VECTORS:");
    println!("  - Total: {:.2} MB ({} GiB)", total_mb, total_mb / 1024.0);
    
    MemoryEstimate {
        total_bytes,
        total_mb,
    }
}

impl EstimateInput {
    pub fn new(dim: usize, count: usize, dtype: DataType, vec_per_dot: usize) -> Self {
        Self {
            dimension: Dimension(dim),
            dot_count: DotCount(count),
            dtype,
            vectors_per_dot: vec_per_dot,
        }
    }
}