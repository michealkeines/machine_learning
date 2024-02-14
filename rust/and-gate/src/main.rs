use tch::Tensor;


fn and_gate(x: Tensor, w: Tensor, b: Tensor) -> Tensor {
    // (w.x) + b
    let matrix_mul = x.matmul(&w) + &b;
    // step-function to map the ouput to 0 or 1
    matrix_mul.heaviside(&Tensor::from_slice(&[1.0]))
}

fn main() {
    let weights = Tensor::from_slice(&[1.0, 1.0]);
    let bias = Tensor::from_slice(&[-1.5]);

    let input = Tensor::from_slice(&[1.0, 1.0]);
    // inputs
    let and: Tensor = and_gate(input, weights, bias);
    println!("output: {}", and);
    
}
