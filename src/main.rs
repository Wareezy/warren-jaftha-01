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

fn calculate_loss(y_pred: Tensor<NdArray, 2>, y_true: Tensor<NdArray, 2>) -> Tensor<NdArray, 1> {
    // Find the difference between predicted and actual values
    let diff = y_pred - y_true;

    // Square the differences to remove negative values
    let squared_diff = diff.clone().mul(diff);

    // Calculate the average of the squared differences (Mean Squared Error)
    squared_diff.mean()
}

fn train(){
    // Set up the device (CPU by default)
    let device = Device::<NdArray>::default();

    // Create a new linear regression model
    let mut model = LinearRegression::new();

    // Initialize the Adam optimizer
    let mut optimizer = AdamConfig::new().init();

    // Generate example training data (10 samples)
    let (x_train, y_train) = generate_data(10);

    // Train the model for 1000 epochs
    for _epoch in 0..1000 {
        // Get predictions from the model
        let y_pred = model.forward(x_train.clone());

        // Calculate the loss
        let loss = calculate_loss(y_pred, y_train.clone());

        // Update model weights using backpropagation
        optimizer.backward_step(&loss);
    }
    // Print a message when training is done
    println!("Training complete!");
}

fn evaluate(model: &LinearRegression) {
    // Generate test data (20 samples)
    let (x_test, y_test) = generate_data(20);

    // Get model predictions and convert them into a Vec<f32>
    let y_pred = model.forward(x_test.clone().reshape([20, 1]))
        .into_data()
        .convert::<Vec<f32>>();

    // Pair each x_test value with its predicted y value for visualization
    let data: Vec<(f32, f32)> = x_test.into_data()
        // Convert x_test tensor into a vector
        .convert::<Vec<f32>>()
        .into_iter()
        // Zip x and y values together
        .zip(y_pred.into_iter())
        .collect();

    // Plot the predicted values on a chart
    // Create a chart of size 100x30
    Chart::new(100, 30, 0.0, 10.0)
        // Plot the data as a line graph
        .lineplot(&Shape::Lines(&data))
        .display(); // Show the chart
}


fn main() {
    println!("Hello, world!");
}
