
everything is point in 2d or 3d space

in linear algebra we almost always the vectors start from the origin

x1 =(1,2)

so we move 2 to x axis, 3 to yaxis

adding to vectors mean

x1 = 1,2 x2 = 3, 4

we move 1 to x axis, 2 to yaxis, the 3 to x axis, 4 to axis


we are just moving the final point in the space in some axis

![[Pasted image 20231224190010.png]]

visually we just place the second vector on the tip of the fist vector to get the addition


![[Pasted image 20231224190036.png]]


multiplication is just doubling hte values

if negative, first flip it then double it



![[Pasted image 20231224190937.png]]

we take 1 units as the base for all the operations with i and j

![[Pasted image 20231224190738.png]]

so all vectors are are scalar that multiple the inital i and j units

![[Pasted image 20231224191400.png]]

if we fix value and change the other, the tip of the resulting endpoints will form a straight line

![[Pasted image 20231224192224.png]]

span is set of all possible places we can reach, which is the entire plane

unless both of the initital vectors lie in the same line

if both thhe vectors are the same span they dont give access to anything new, so we call them linearly dependent

if both the vectors different, it give access to new span, saying linearly independent

![[Pasted image 20231224193138.png]]

linear transformation means, we are moving a vector from one point to another

![[Pasted image 20231224193339.png]]

it is a linear transformation, only if it satisfies these two properties


![[Pasted image 20231224195252.png]]


in a transformation

the input columns are the corridinates for where i and j hats will land after a transformation

so given a vector, we can calculate how this transformation will change the input vector

![[Pasted image 20231224195510.png]]

this is the intuation behind the matrix mutiplcation

the input is just the transformation, the mutiplicant is the input vector, so we find what that transformation will do the input vector



![[Pasted image 20231224200059.png]]


the input are already transformed vectors in space, now we apply the input vector to reach the final transformation

 this is what excatly happens behind a matrix multiplication


example 

let take base i (1, 0) j (0, 1)

this iis the basic units

if we do a 90 degree transformation,

i will be (0, 1) j (-1, 0)


so if we have an vector, and we want to do a 90 degree transformation, we just multiple that vector with this transformations, we get the position, we a 90 degree transformation will take that input vector

![[Pasted image 20231224200844.png]]



multiplication of matrix is just applying one transformation at a time

![[Pasted image 20231224203656.png]]

rotation wth first column

![[Pasted image 20231224203722.png]]


rotation with second column

![[Pasted image 20231224203825.png]]


this is what we memorized a matrix multiplication with an algorthmic way

![[Pasted image 20231224203914.png]]

determinant of transformation is the scale of the area increase,

that is is if the transformation increases the total area by a factor of 6, then the determiant is 6

![[Pasted image 20231224205211.png]]


if the transfomation quiches everything into single point, then the determiant is 0

![[Pasted image 20231224205412.png]]

that is in case of 2 dimemsiosal transformation, the transformation will be a line

it brings down the dimensions

![[Pasted image 20231224205716.png]]

if the determinant is negative, then the area is scaled by the same factor but the the location is flipped

in 3d detertimanant is scaled the same way, but also the volume is scaled in the 3rd axis

![[Pasted image 20231224210517.png]]

after tranformation




![[Pasted image 20231224210558.png]]


calculating determinant

![[Pasted image 20231224210925.png]]


![[Pasted image 20231224211221.png]]

linear system of equations

![[Pasted image 20231224211805.png]]

Martix A corresponds to a linear transformation

vector v is output vector

we want to find a transfformation htat will transform the input transformation to vector v

![[Pasted image 20231224212008.png]]

if the determinant is non zero

![[Pasted image 20231224212138.png]]

there will always be a vector that can transform that input in ot the output vector

![[Pasted image 20231224212342.png]]

this is where find the inverse helps

![[Pasted image 20231224214322.png]]


we are applying the inverse  transformation ot the final transformation ot get it back o the applied transformation

![[Pasted image 20231224220839.png]]


we have ranks to define how a transfomation is transformed into

if it sequihes the intial trasnformation into a line, then rank is 1

![[Pasted image 20231225153202.png]]



if that sequiches into a two dimensional plane, thn rank is 2

![[Pasted image 20231225153252.png]]



if 3 diemsinal input that can max ran of 3

if 2 dimensiona inpu can have max rank of 2


if rank 3 sequiches into rank 1, then finding a solution may need differenet method

![[Pasted image 20231225153458.png]]

that is all the possible transformations like line, plane or point from an initial transformation is called a column space of the initial transformation A

 what is an column space


![[Pasted image 20231225155910.png]]

![[Pasted image 20231225155929.png]]

![[Pasted image 20231225155946.png]]

![[Pasted image 20231225155959.png]]

the null space encompasses all the vectors that "do nothing" under the transformation represented by matrix A. They are the silent, invisible, or "null" elements with respect to the action A is performing.

The null space (also known as the kernel) of a matrix A is a concept in linear algebra that represents all the vectors that, when multiplied by A, result in the zero vector. It's a subspace of the domain of the transformation described by A.

Here's an intuitive way to understand the null space:

### Intuition for Null Space:

1. **Shadow or Footprint**:
    
    - Imagine shining a light through a set of shapes onto a surface. The shadow (or footprint) that lands on the surface represents the column space; it's what's visible or has an effect. Now, if there are parts of the shapes that do not cast a shadow because they are in line with the light, these parts are analogous to the null space. They are there, but they don't leave a mark on the surface—the column space.
2. **Directions Leading Nowhere**:
    
    - Think of the null space as directions you can move in that won't change your position in terms of the transformation AA. If you're standing on a flat surface and this surface is the column space, then stepping "up" or "down" wouldn't move you anywhere on that surface; it's a movement that doesn't affect your position on the plane.

![[Pasted image 20231225161252.png]]

![[Pasted image 20231225161311.png]]


the null space deals with inputs that lead to no output, while the column space deals with possible outputs from various inputs.


if the input transformation is not square, then the ijnput is converted from 2d to 3d or 3d to 2d



Dot product

order of mulitplication doesnt matters, if v is projected on w or w on v



![[Pasted image 20231225175716.png]]

![[Pasted image 20231225180136.png]]

properties

![[Pasted image 20231225180340.png]]

if they are pointing at tehe same direction relatively

![[Pasted image 20231225180419.png]]

if thye are perpendicular then the its zero

![[Pasted image 20231225180503.png]]

if pointing generally to the opposite, then the product is negative

if we transform a 2d vector to 1d line

 and if this is a linear transformation, the units will stay the same

![[Pasted image 20231225181855.png]]

example if we do a linear transfomration of 

![[Pasted image 20231225182154.png]]

so our intial transformation is 1, -2

so if we apply this transformation on a vector 4, 3

they will land on 4 times i and 3 time j

![[Pasted image 20231225182308.png]]

so it lands on -2

![[Pasted image 20231225182355.png]]

this transformations looks excatly like a taking dot product of vectors

![[Pasted image 20231225182518.png]]

![[Pasted image 20231225182616.png]]

1x2 matrices can be transposed to get the vectors that might give that tranformation

some connection between linear transformations from

vectors to numbers

![[Pasted image 20231225182822.png]]


2,3 as vector in 2d space can be fliped to a transformation of 2,3 in 1d plane


![[Pasted image 20231225183048.png]]


place a 1d line segment in the diagonal of the 2d space

the 2d vectors can be converted to id value fail in the diagonal line

![[Pasted image 20231225183356.png]]

if project the u on the x axis and poject i into the diagonal, the place where u lands will be equal to where x lands in u

so we know the ux and uy

![[Pasted image 20231225183538.png]]


so to get the transformation of arbirtary vector

![[Pasted image 20231225183902.png]]

we just multiple ux, uy into that vector, which is indential to taking a do product

![[Pasted image 20231225184001.png]]

this is just one duality

![[Pasted image 20231225184033.png]]

both give same output

![[Pasted image 20231225184233.png]]

it is just that where we jhave 2d-to-1d linear transformation, there will always be vector in 2d space


cross product


![[Pasted image 20231225185411.png]]


cross product is the area of the transformation

so we just have to find the determinant to get the value

for 2d it is just ad - bc

![[Pasted image 20231225185527.png]]

also if the base vectors are is left of the v, that is think about x, y, bases, x is right of the y to begin with

so if w is to the left of v, then it is negative area

so v x w = - (v x w)

in 3d cross product, the final output wilil be a vector

![[Pasted image 20231225190201.png]]

![[Pasted image 20231225190242.png]]

we add the bases i,j,k to get the vector form of this final value

carmers rule

![[Pasted image 20231225192023.png]]

this is how we foudn the solutions initially

![[Pasted image 20231225192609.png]]

![[Pasted image 20231225192849.png]]

changing from one basis to another

![[Pasted image 20231225193119.png]]

we just add the scalars to our base

![[Pasted image 20231225193221.png]]

![[Pasted image 20231225193357.png]]

we can use that vector transform our basis vector 1,0, 0,1 to reach her basis

eigen vectors



in general if we take a random vector, for given transformation, the vector will get thrown off it course due to that transfformation

![[Pasted image 20231225195523.png]]

what if we can find a vector that even after the tranformation is in the same span, but streached or squeeched but in the same course

![[Pasted image 20231225195419.png]]

so anything on that space can be the eigen vector

in this example 3,0 is a eigen vector as it streches along the x axis, 4, 0 is also an eigne vector as it will aso oonly get stretched

![[Pasted image 20231225195933.png]]

![[Pasted image 20231225200010.png]]

eigen value is the factor by which it is streched or squeeced during this transformation

![[Pasted image 20231225200225.png]]

![[Pasted image 20231225200329.png]]

so we are looking for a vector time this transformation with lamda gives zero

![[Pasted image 20231225200602.png]]

we cna ble think

![[Pasted image 20231225200706.png]]

this is true, because if the determiant of an tranformation is 0, then it is squeeing thing the area into lower dimension

![[Pasted image 20231225200832.png]]

this is how is is calculted

![[Pasted image 20231225201307.png]]


![[Pasted image 20240109161828.png]]

![[Pasted image 20240109162112.png]]

![[Pasted image 20240109162237.png]]

Principal Component Analysis

![[Pasted image 20240109164852.png]]

we want measure the spread of these numbers

![[Pasted image 20240109165234.png]]

that is to sum every points and divide it by total count

![[Pasted image 20240109165446.png]]

now found the average distance of point from the mean

this method of taking mean and then average from the mean is called the variance

![[Pasted image 20240109170556.png]]

we just take mean for both point that forms a vector respect ot the mean vector

![[Pasted image 20240109170722.png]]

but then we have many possible lines that can pass through that mean vector

![[Pasted image 20240109173248.png]]

![[Pasted image 20240109173331.png]]

![[Pasted image 20240109173230.png]]

for higher dimensions

![[Pasted image 20240109173408.png]]

![[Pasted image 20240109173510.png]]

![[Pasted image 20240109173802.png]]

![[Pasted image 20240109173929.png]]



