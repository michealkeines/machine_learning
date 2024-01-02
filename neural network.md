
divide the problem into small task

calculate normal determenistic method to find something and set that as driver for the particular neuron

but we also add a bias value in the end

this is the one we can tweak to come up final value of bias for this neuron, that will hopefull work well

![[Pasted image 20240102145625.png]]


if we do this for all the small task and finally input the input data, we get a final results

![[Pasted image 20240102150027.png]]

gradient descent

we get the final error cost, this cost (current - actual) is graphed, and 

think of it as chain rule in caculus

everry layer is affecting each other like the chain rule, so if we find the local minimum for the cost funtion, it is equal to finding the minium weight and bias for the instance of training data

![[Pasted image 20240102161436.png]]

![[Pasted image 20240102161512.png]]

derivate of all there influces the final cost

diff cost by weight, ie, how small difference in weight influence final C

![[Pasted image 20240102161733.png]]



take a point in the cost function, find the slope, that is the derivative

if the slope is positive, go left
if the slope is negative, go right

![[Pasted image 20240102151607.png]]

backpropogation

because we know the final value which needs to be higher, we just go from the final layer backwards, nudging the weights  and bias 

![[Pasted image 20240102153621.png]]

