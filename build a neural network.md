
![[Pasted image 20240127175733.png]]

![[Pasted image 20240127181041.png]]

![[Pasted image 20240209163642.png]]

![[Pasted image 20240209164106.png]]

![[Pasted image 20240209170015.png]]

the step function, 

![[Pasted image 20240209170407.png]]

all vectors for x that satisfies this equation will be lying on the plane

Perceptrons

a perceptron combines the step function and hyper plane

this particular function maps all point on one side of the plane to zero and other side of the plane to one


this way we figure where our new point belong

![[Pasted image 20240209171852.png]]

```python

import torch

def fully_connected_layer(X, W, b):
    X = torch.cat((X, torch.ones([X.shape[0], 1], dtype=torch.float32)), dim=1)
    W = torch.cat((W, b.unsqueeze(dim=1)), dim=1)
    y = torch.matmul(W, X.transpose(0, 1)) # get matrix multiplication
    print(y)
    y = torch.heaviside(y, torch.tensor(1.0)) # if it is negative, we will get zero, positive, one

    return y.transpose(0, 1)

def perceptron(x, w, b):
    return fully_connected_layer(x, w, b)

and_f = perceptron(torch.tensor([[1,1]]), torch.tensor([[1, 1]]), torch.tensor([-1.5]))
print(and_f)

                         0 0 = 0
                         0 1 = 0                         
                         1 0 = 0
                         1 1 = 1


           
```


![[Pasted image 20240209183737.png]]


xor needs two perceptrons

![[Pasted image 20240211135109.png]]

![[Pasted image 20240211135048.png]]

Layering multiple perceptrons into a network is called multi layer perceptron.

![[Pasted image 20240212145016.png]]



but the thing is step functions are not differentiable



we want something that can mimci step function also differntiable

sigmoid function is the best alternative

![[Pasted image 20240212150700.png]]

it is the S curve

![[Pasted image 20240212150722.png]]

![[Pasted image 20240212150925.png]]

![[Pasted image 20240212151323.png]]

![[Pasted image 20240212152126.png]]

weights are specific to this layer, a(l-1) is the output of the previous layer

if every perceptron from the previous layer is connected to every perceptron in the next layer, such a layer is known as fully connected layer.

![[Pasted image 20240212160843.png]]

if we start from the layer 0 and keep pushing inward, we call it forward propagation

at a given time only current and previous layer will be in the memory

ground truth, is the actual output we are excpeted, 

![[Pasted image 20240213133424.png]]

Loss and Its Minimization

![[Pasted image 20240213134057.png]]

![[Pasted image 20240213134258.png]]


![[Pasted image 20240213134443.png]]


![[Pasted image 20240213134502.png]]

forward propagation for arbitrary layer

![[Pasted image 20240213164721.png]]

this will be last output of the layers as we calculate it one after the other

with every forward movement, the output from the previous layer is used to caculate the current value

after the final layer, the loss is calculated base on final layers output and ground truth for that particular training instance

![[Pasted image 20240213165622.png]]



if we go from layer 0 to layer n - forward pass

if we go from layer n to layer 0 - back progagation


gradiant descent is just we have loss equation, with variable that can be manipulated

if loss output = 10 and ground truth is 15

so to change the value of the variables, we cant just subtract the total 5, as all the variables combine to give output 10, they are interdepending, so we need a way to find the poper minium value that we can subtract that will reduce the over loss ouput closer to ground truth

if we have function it can be plotted and 

and all the variabe come into place in that curve, so if we want to find the change in loss after reducing by some value, we need to take derivate of it, but we can change the variables indiviually, so we can use partial derviate to find the change per variable, and change will be minium if we go along the slope, this is what we do in gradiant disant

![[Pasted image 20240213181212.png]]

![[Pasted image 20240215174509.png]]

![[Pasted image 20240215181846.png]]

The gradient is nothing but a vector of partial derivatives.

![[Pasted image 20240217125621.png]]

for a single neuron, to get activation value, we need to use activation value from all the neurons from the previous layer

![[Pasted image 20240217130348.png]]


![[Pasted image 20240217132420.png]]

learning rate r is multipled to the new weight and bias

