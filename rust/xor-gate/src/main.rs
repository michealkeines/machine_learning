use tch::{Tensor};

fn Z(x: &Tensor, w: &Tensor, b: &Tensor) -> Tensor {
    x.matmul(w) + b
}

fn A(z: &Tensor) -> Tensor {
    Tensor::sigmoid(z)
}

fn forward_propagation(x: Tensor, all_w: Tensor, all_b: Tensor) -> Tensor {
    let mut current_a: Tensor = x;

    for l in 0..3 {
        let temp: Tensor = current_a.copy();
        let z = &Z(&temp, &all_w.get(l), &all_b.get(l));
        current_a = A(z);
    }
    current_a
}

fn mse_loss(a_L: Tensor, y: Tensor) -> Tensor {
    1./ 2 * (a_L - y).pow(2)
}

fn forward_backward(x: Tensor, y: Tensor, w: Tensor, b: Tensor) {
    let L = w.size()[0] - 1;
    let mut a: Vec<Tensor> = vec![];

    for l in 0..(L + 1) {
        a_prev = match l {
            0 => x,
            _ => a[l-1]
        };

        let z_l = Z(&a_prev, &w[l], &b[l]);
        let a_l = A(z_l);
        a.push(a_l.copy());
    }
    
    println!("Final Activation: {}", a[L]);
    let loss = mse_loss(a[L], y);
    println!("Loss: {}", loss);

    let mut deltas = vec![None] * (L + 1);
    let mut w_grads = vec![None] * (L + 1);
    let mut b_grads = vec![None] * (L + 1);

    // compute the last layer
    let mut a_L: Tensor = a[L];
    deltas[L] = (a_L - y) * a_L * (1 - a_L);
    w_grads[L] = a[L - 1].T.matmul(deltas[L]);
    b_grads[L] = deltas[L];

    for l in rev(0..(L-1)) {
        a_L = a[l].copy();
        deltas[l] = w[l+1].T.matmul(deltas[l+1]) * a_l * (1 - a_l)
        w_grads[l] = deltas[l].matmul(a[l - 1].T)
        b_grads[l] = deltas[l]
    }
}

 


fn main() {
    let weight = Tensor::from_slice(&[1.0, 1.0]);
    let input_layer = Tensor::from_slice(&[0.5, 0.6]);    
    let hidden_layer = Tensor::from_slice(&[0.4, 0.2]);
    let output_layer = Tensor::from_slice(&[0.4]);

    let temp = input_layer.copy();

    println!("input: {}, weight: {}, temp: {}", input_layer, weight, temp);
}
