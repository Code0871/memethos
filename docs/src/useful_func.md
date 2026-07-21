# Useful Fnctions

# Calculation of required memory
Since we need to determine how much data will be loaded from disk into RAM to accelerate vector searches, we wrote a calculation function *estimate()*; currently, it calculates the amount of memory that needs to be allocated *only* for the vectors

Usage example:
```Rust
// import
use mindmap::estimate::estimate::*;

let input = EstimateInput::new(
    768,            // dimension of vector
    1_000_000,      // vector_count
    DataType::I8,   // dtype (4 bytes)
    3,              // vectors_per_dot
);

estimate(input);

//output:
// 📊 Memory usage ONLY FOR VECTORS:
//   - Total: 2197.27 MB (2.1457672119140625 GiB)
```
Future plans include making the calculation more flexible to allow for specifying a wider range of options for storing vectors associated with a single point