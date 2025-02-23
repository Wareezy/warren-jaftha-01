use rand::{Rng, thread_rng};
use burn::tensor::{Tensor, Data, Device};
use burn_ndarray::NdArray;
use burn::nn::{Linear, LinearConfig};
use burn::optim::{Adam,AdamConfig};
use burn::prelude::TensorData;
use textplots::{Chart, Plot, Shape};

// Define a simple Linear Regression model
struct LinearRegression {
    // This is our single-layer model (just one linear transformation)
    layer: Linear<NdArray>,
}

impl LinearRegression {
    // Function to create a new LinearRegression model
    fn new() -> Self {
        // Set up the device (CPU by default)
        let device = Device::<NdArray>::default();

        // Create a linear layer with one input and one output
        let layer = LinearConfig::new(1, 1).init(&device);

        // Return an instance of the model
        Self { layer }
    }

    // Function to make predictions (forward pass)
    fn forward(&self, x: Tensor<NdArray, 2>) -> Tensor<NdArray, 2> {
        // Pass the input through the layer and return the output
        self.layer.forward(x)
    }
}


fn main() {
    println!("Hello, world!");
}
