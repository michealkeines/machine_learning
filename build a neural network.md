
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

