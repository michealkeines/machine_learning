
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

![[Pasted image 20240108151828.png]]

![[Pasted image 20240108152020.png]]


finding if a funtion is or convex and non convex wil help us find the minimas

![[Pasted image 20240108152808.png]]

If we take any pair of of points in the green region and join them with a straight line, all points on that line will also be in the green region. so convex

a set of points is non-convex if it contains at least one pair of points whose joining line contains a point not belonging to the set

the boundary of a convex set is always as convex curve

finding second derivate of a funtion, if it is positive, it is convex

![[Pasted image 20240108160852.png]]

![[Pasted image 20240108160947.png]]

You start with a curve and pick two points on it, let's call them AA and BB.
Then you find a point CC which is the weighted average of AA and BB, lying on the straight line that connects them.
The x-value of CC is the same mix (weighted average) of the x-values of AA and BB.
Now, if you drop a vertical line from CC to hit the curve, you find Point DD.

If Point DD is directly below CC (or at the same spot as CC if the curve passes through CC), then the curve is convex at least between AA and BB. This means the segment of the curve between AA and BB does not rise above the straight line connecting AA and BB. That's a signature property of a convex curve in that region.

A little thought will reveal that definition 1 implies that convex curves always curls upwards and/or rightwards, everywhere. This leads to another equivalent definition of convexity.

A function that curves upwards everywhere is always going to lie above its tangent. This leads to another equivalent definition of a convex function

![[Pasted image 20240108161433.png]]

![[Pasted image 20240108161441.png]]

![[Pasted image 20240108163420.png]]



![[Pasted image 20240108163501.png]]


