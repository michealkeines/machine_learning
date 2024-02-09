
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
           
```


![[Pasted image 20240209183737.png]]



