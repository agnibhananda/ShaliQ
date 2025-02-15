
- Brains are made up of neurons connected via synapses
- Synapses carry electric and chemical signals between neurons
- A synapse is like a wire that connects one neuron's output to another neuron's input; each synapse is of certain weight
- Neurons decide whether to go ahead or stop the signal


## Feed Forward Neural Network
FNNs work by accepting some numbers at their input and propagating (feedforwarding) those numbers layer-by-layer across the entire network; numbers that appear at the last layer determine network's answer. 

### Bias
Bias is a parameter that controls the threshold for neuron activation. It allows the neuron to fire even if the sum of its inputs is below the threshold.

Imagine you've got a neuron with three inputs, with each input determining whether it sees bird (1.0) or not (0.0) - now, if you wanted to create a neuron that's activated when it sees at least two birds, you'd simply create a neuron with a bias of -1.0; this way your neuron's "natural" state would be -1.0 (inactive), with one bird - 0.0 (still inactive), and with two - 1.0 
