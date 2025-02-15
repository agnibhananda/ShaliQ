use rand::rng;

pub struct Network{
    layers: Vec<Layer>,
}
pub struct LayerTopology{
    pub input_neurons: usize,
    pub output_neurons: usize,
}
struct Layer{
    neurons: Vec<Neuron>,
}
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}


impl Network{
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
            self.layers
                .iter()
                .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
    pub fn random(layers: Vec<LayerTopology>) -> Self {
        let mut built_layers = Vec::new();
        for i in 0..(layers.len() - 1){
            let input_size=layers[i].neurons;
            let output_size=layers[i+1].neurons;
            built_layers.push(Layer::random(
                input_size,
                output_size,
            ));
        }

        Self { layers: built_layers }
    }

}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
        .iter()
        .map(|neuron| neuron.propagate(&inputs))
        .collect()
    }
    fn random(input_size: usize, output_size: usize) -> Self {
        let mut neurons = Vec::new();
        for _ in 0..output_size {
            neurons.push(Neuron::random(input_size));
        }
        Self {neurons}
    }
}

impl Neuron{
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
    fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}