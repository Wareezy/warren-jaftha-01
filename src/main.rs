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

fn generate_data(n: usize) -> (Tensor<NdArray, 2>, Tensor<NdArray, 2>) {
    // Set up the device (CPU by default)
    let device = Device::<NdArray>::default();

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a list of random x values between 0 and 10
    let x: Vec<f32> = (0..n).map(|_| rng.gen_range(0.0..10.0)).collect();

    // Generate y values using the equation y = 2x + 1 with some random noise
    let y: Vec<f32> = x.iter().map(|&x| 2.0 * x + 1.0 + rng.gen_range(-1.0..1.0)).collect();

    // Convert x and y values from Vec<f32> into tensors
    let x_tensor = Tensor::<NdArray, 2>::from_data(TensorData::from(x.as_slice()), &device);
    let y_tensor = Tensor::<NdArray, 2>::from_data(TensorData::from(y.as_slice()), &device);

    // Return the generated x and y tensors
    (x_tensor, y_tensor)
}



fn main() {
    println!("Hello, world!");
}
