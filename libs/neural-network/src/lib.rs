pub struct Network{
    layers: Vec<Layer>,
}

pub struct LayerTopology{
    pub input_neurons: usize,
    pub output_neurons:usize,
}

struct Layer{
    neurons: Vec<Neuron>,    
}

struct Neuron{
    bias: f32,
    weights: Vec<f32>,
}

impl Network{
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32>{
        
        for layer in &self.layers{
            inputs = layer.propagate(inputs);
        }

        inputs//return statement
    }
}

impl Layer{
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32>{
        let mut outputs = Vec::new();

        for neuron in self.neurons{
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }
        outputs
    }
}

impl Neuron{
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0);
    }
}

impl LayerTopology{
    pub fn random(layers: &[LayerTopology]) -> Self{
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            })
            .collect();

        Self { layers }

    }

}
