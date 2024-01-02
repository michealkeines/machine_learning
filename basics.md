decide the q uestion we are trying to solve, why can we just write rule based program for this, then decide if a model is required

we have a problem and we want find a line that will spearatte the input into separtable clusters

to plot them, first we need to choose the features that we are gonna consider for a porblem

this is important and highly related to how well this is gonna do, choosing something specific is really importtant

sometime we just tolarate the number of false positive and decrease the model till there is no false positive

we need to find the best smallest distance to group points,

popular choices, manhatten, ecledian

![[Pasted image 20231210175500.png]]

if we overfit, then we didnt learn anything, we just pratically programmed a line for every input

![[Pasted image 20231210180139.png]]

if a data is there, k-means method will find the k nearest elements and check if what are their label, if all of them are same, we make it as same if not mark is as unknown

![[Pasted image 20231210180709.png]]

![[Pasted image 20231210181047.png]]

if you increase the sensitivity we mark every unknown as what we want, then we are markking some thing wrong

if we increase the specifigy we dont make every unknwn as what we want, then we are not marking enough 

this can seen visuallly using a receiver operator characterstic curve to see the thresholds of different lines

i guess we can use mulitple algorithms find the match for a single model

feature extraction is process of taking features form the dataset

feature selection is process of select a subset of feature that already exist

cosine agreement

compaing two vector using dot product is equal to finding cosine of a x b

Expressing the dot product using cosines makes it easier to see that it measures the _agreement_ (aka _correlation_) between two vectors. If the vectors have the same direction, the angle between them is 0 and the cosine is 1 implying maximum agreement. The cosine progressively becomes smaller as the angle between the vectors increases until the two vectors become perpendicular to each other when the cosine becomes zero, implying no correlation - vectors are independent of each other. If the angle between them is 180∘ the cosine is -1 implying vectors are anti-correlated. Thus, the dot-product of two vectors is proportional to the directional agreement between them.

![[Pasted image 20231224164240.png]]

orthogonal vectors are independent to each other

v = (1, 0) w = (0, 1)

these two input feature vectors have nothing in common, they are orthogonal to eacho other, that is the dot.product is zero


In real life situations, the separator is often a line or a plane in a high dimensional space. In more complicated situations, the separator is a curved surface

types of separators, lines and planes in high dimensional spaces, aka hyper-lines and hyper-planes.

![[Pasted image 20231228201719.png]]

![[Pasted image 20231228201821.png]]

any point in the line can be given using this equation

![[Pasted image 20231228202028.png]]

span of vectors

![[Pasted image 20231228205010.png]]

tensors


A vector can be viewed as a 1 tensor, a matrix is 2 tensor, a scalar is 0 tensor.

