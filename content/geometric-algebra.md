+++
title= "Fantastic quaternions and where to find them"
date        = "2017-12-30"
+++

## Wedge product
The wedge product will give us there area that two vectors are spanning.

We start of with two properties:

$$ \vec{a} \wedge b = -(b \wedge a) $$

The wedge product will give us the signed area, just like the determinant.

$$ a \wedge a = 0$$

And if two vectors are the same or a multiple of each other, they will have an area of 0. With these two properties, we can derive the actual formula to calculate the area that two vectors are spanning.


$$ u = a\vec{x} + b\vec{y}$$ 

$$ v = c\vec{x} + d\vec{y}$$

$$ u \wedge v = (a\vec{x} + b\vec{y}) \wedge (c\vec{x} + d\vec{y}) $$
$$ u \wedge v = a\vec{x} \wedge c\vec{x} + a\vec{x} \wedge d\vec{y} + b\vec{y} \wedge cx + b\vec{y} \wedge d\vec{y}$$
$$ u \wedge v = a\vec{x} \wedge d\vec{y} + b\vec{y} \wedge c\vec{x}$$
$$ u \wedge v = a\vec{x} \wedge d\vec{y} - c\vec{x} \wedge b\vec{y}$$
$$ u \wedge v = ad (\vec{x} \wedge \vec{y})  - bc(\vec{x} \wedge \vec{y})$$
$$ u \wedge v = (ad - bc) \vec{x} \wedge \vec{y}$$
This is essentially the 2d determinant times $\vec{x} \wedge \vec{y} $. The area of $\vec{x} \wedge \vec{y} $ is 1.


The wedge product is also associative.
$$ (\vec{u} \wedge \vec{v}) \wedge \vec{w} = \vec{u} \wedge (\vec{v} \wedge \vec{w}) $$
You can imagine $(\vec{u} \wedge \vec{v})$ to span a plane in 3d space and $\wedge \vec{w}$ will extend this plane into a 3 dimensional object.

![](https://i.imgur.com/RNYTzJe.gif)

It doesn't matter which plane we create first because all will have the same volume in the end. The wedge product is define for `N` dimensions but we will only use 2d and 3d in this blog post.
## Geometric product
$$\vec{a}\vec{b} = \vec{a} \cdot \vec{b} + \vec{a} \wedge \vec{b} $$

The geometric product is just the dot product + the wedge product.

The geometric product is both commutative and anti-commutative.

If $\vec{u}$ and $\vec{v}$ are orthogonal, then the dot product becomes 0 and we are left with the wedge product
$$ \vec{u}\vec{v} = \vec{u} \wedge \vec{v}$$
and therefor is anti-commutative
$$ \vec{u}\vec{v} = -\vec{v}\vec{u}$$

If $\vec{u}$ and $\vec{v}$ are linearly dependent, then the wedge product becomes 0 and we are left with only the dot product.
$$ \vec{u}\vec{v} = \vec{u} \cdot \vec{v}$$
$$ \vec{u}\vec{v} = \vec{v}\vec{u}$$


$$ \vec{x}\vec{x} = \vec{x} \cdot \vec{x} = 1 $$

You can also write $ \vec{x}\vec{x} $ as 

$$ \vec{x}\vec{x} = \vec{x}^2 $$
$$ \vec{x}^2 = \vec{x}\vec{x} = \vec{x} \cdot \vec{x} = 1 $$


Now we will look at the geometric product of $\vec{x}\vec{y} $.
$$ \vec{x}\vec{y} = \vec{x} \wedge \vec{y}$$

Because of this we get

$$ \vec{x}\vec{y} = -(\vec{y}\vec{x})$$
$$ (\vec{x}\vec{y})^2 = \vec{x}\vec{y}\vec{x}\vec{y}$$

Remember order matters.


$$ (\vec{x}\vec{y})^2 = \vec{x}(-\vec{x}\vec{y})\vec{y} = -\vec{x}\vec{x}\vec{y}\vec{y} = -\vec{x}^2\vec{y}^2 = -1$$

Interestingly the squared geometric product of the basis vectors will result in $-1$. This is very similar to $i^2 = -1$. You might wonder if we can do rotations with $\vec{x}\vec{y}$ and the answer is yes we can.

$$ \vec{u} = a\vec{x} + b\vec{y} $$
$$ \vec{u}\vec{x}\vec{y} = 
(a\vec{x} + b\vec{y})\vec{x}\vec{y} = a\vec{x}\vec{x}\vec{y} + b\vec{y}\vec{x}\vec{y} = a\vec{x}\vec{x}\vec{y} - b\vec{x}\vec{y}\vec{y} = - b\vec{x} + a\vec{y} $$

You can compare this to 

$$ (a + bi)i = ai + bi^2 = -b + ai $$

Multiplying a vector $\vec{u}$ with $\vec{x}\vec{y}$ will result in a vector, and $(\vec{x}\vec{y})^2$ will result in a scalar.


Before we put $\vec{x}\vec{y}$ on the right side and we did a $90\degree$ counter clockwise rotation. What happens if you put $\vec{x}\vec{y}$ on the left side?
$$ \vec{x}\vec{y}\vec{u} = \vec{x}\vec{y}(a\vec{x} + b\vec{y}) = \vec{x}\vec{y}a\vec{x} + \vec{x}\vec{y}b\vec{y} 
=a(\vec{x}\vec{y}\vec{x}) + b(\vec{x}\vec{y}\vec{y}) = b\vec{x} -a\vec{y}$$

It seems that if $\vec{x}\vec{y}$ is on the left side we will do a $90\degree$ clockwise rotation. 
Lets see what happens if we swap $\vec{x}\vec{y}$ to $\vec{y}\vec{x}$
$$ \vec{y}\vec{x}\vec{u} = \vec{y}\vec{x}(a\vec{x} + b\vec{y}) = \vec{y}\vec{x}a\vec{x} + \vec{y}\vec{x}b\vec{y} 
=a(\vec{y}\vec{x}\vec{x}) + b(\vec{y}\vec{x}\vec{y}) = - b\vec{x} + a\vec{y} $$

We know that $(\vec{x}\vec{y})^2$ is doing a $180\degree$ rotation because we multiply by $-1$. The same is true for $(\vec{y}\vec{x})^2$ because we just rotate from the other side. If $\vec{x}\vec{y}$ is a $ 90 \degree $ clockwise rotation and $\vec{y}\vec{x}$ $90\degree$ counter clockwise rotation, then we would assume that combining them would result in $1$.

$$\vec{u}\vec{x}\vec{y}\vec{y}\vec{x} = \vec{u}\vec{x}\\vec{x} = \vec{u}$$

If we put $\vec{x}\vec{y}$ and the left and right side, we would also expect no change.

$$ \vec{u} = a\vec{x} + b\vec{y} $$
$$\vec{x}\vec{y}\vec{u}\vec{x}\vec{y} = \vec{x}\vec{y}(a\vec{x} + b\vec{y})\vec{x}\vec{y} $$ 
$$ = (\vec{x}\vec{y}a\vec{x} + \vec{x}\vec{y}b\vec{y})\vec{x}\vec{y}$$
$$ = (a\vec{x}\vec{y}\vec{x} + b\vec{x}\vec{y}\vec{y})\vec{x}\vec{y}$$
$$ = (-a\vec{y} + b\vec{x})\vec{x}\vec{y}$$
$$ = -a\vec{y}\vec{x}\vec{y} + b\vec{x}\vec{x}\vec{y}$$
$$ = a\vec{x} + b\vec{y} = \vec{u}$$

## Inverse

$$ \vec{u}\vec{u} = \vec{u} \cdot \vec{u} = |\vec{u}|^2$$
$$ \vec{u}\vec{u} = |\vec{u}|^2 $$
$$ \frac{\vec{u}}{|\vec{u}|^2}\vec{u} = 1 $$
$$ \vec{u}^{-1} = \frac{\vec{u}}{|\vec{u}|^2}$$
$$ \vec{u}^{-1}\vec{u} = 1$$


## Dot product
$$\vec{u}\vec{v} = \vec{u} \cdot \vec{v} + \vec{u} \wedge \vec{v} $$
$$\vec{v}\vec{u} = \vec{v} \cdot \vec{u} + \vec{v} \wedge \vec{u} = \vec{u} \cdot \vec{v} - \vec{u} \wedge \vec{v} $$
$$\vec{u}\vec{v} + \vec{v}\vec{u} = 2(\vec{v} \cdot \vec{u}) $$
$$(\vec{v} \cdot \vec{u}) = \frac{\vec{u}\vec{v} + \vec{v}\vec{u}}{2}$$

## Wedge product

$$\vec{u}\vec{v} = \vec{u} \cdot \vec{v} + \vec{u} \wedge \vec{v} $$
$$\vec{v}\vec{u} = \vec{v} \cdot \vec{u} + \vec{v} \wedge \vec{u} = \vec{u} \cdot \vec{v} - \vec{u} \wedge \vec{v} $$
$$\vec{u}\vec{v} - \vec{v}\vec{u} = 2(\vec{u} \wedge \vec{v})$$
$$\vec{u} \wedge \vec{v} = \frac{\vec{u}\vec{v} - \vec{v}\vec{u}}{2} $$


## Reflection

![](https://i.imgur.com/drCMAoR.png)

$$ \vec{u} = \vec{u}\_\perp + \vec{u}\_{\parallel} $$
$$ \vec{u}^{\prime} =\vec{u}\_{\parallel} - \vec{u}\_\perp$$
$$ \vec{u}^{\prime}\vec{v}\ = \vec{u}\_{\parallel}\vec{v}\ - \vec{u}\_\perp\vec{v}$$
$$ \vec{u}^{\prime}\vec{v}\ = \vec{v}\vec{u}\_{\parallel} + \vec{v}\vec{u}\_\perp$$
$$ \vec{u}^{\prime}\vec{v}\ = \vec{v}\vec{u}\_{\parallel} + \vec{v}\vec{u}\_\perp$$
$$ \vec{u}^{\prime}\vec{v}\ = \vec{v}(\vec{u}\_{\parallel} + \vec{u}\_\perp)$$
$$ \vec{u}^{\prime}\vec{v}\ = \vec{v}\vec{u}$$
$$ \vec{u}^{\prime}\vec{v}\\vec{v}^{-1} = \vec{v}\vec{u}\vec{v}^{-1}$$
$$ \vec{u}^{\prime} = \vec{v}\vec{u}\vec{v}^{-1}$$
Interestingly we could have also used the opposite approach.
$$ \vec{u} = \vec{u}\_\perp + \vec{u}\_{\parallel} $$
$$ \vec{u}^{\prime} =\vec{u}\_{\parallel} - \vec{u}\_\perp$$
$$ \vec{v}\vec{u}^{\prime} = \vec{v}\vec{u}\_{\parallel} - \vec{v}\vec{u}\_\perp$$
$$ \vec{v}\vec{u}^{\prime} = \vec{u}\_{\parallel}\vec{v} + \vec{u}\_\perp\vec{v}$$
$$ \vec{v}\vec{u}^{\prime} = (\vec{u}\_{\parallel} + \vec{u}\_\perp)\vec{v}$$
$$ \vec{v}\vec{u}^{\prime} = \vec{u}\vec{v}$$
$$ \vec{v}^{-1}\vec{v}\vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v}$$
$$ \vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v}$$

## Double reflection as a rotation
![](https://i.imgur.com/Kzp6tzy.png)

Instead of reflecting $\vec{u}$ along $\vec{v}$, we will introduce another vector $\vec{w}$ and do a double reflection.

To get t he first reflection we use the equation that we derived above.


$$ \vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v}$$
$$ \vec{u}^{\prime\prime} = \vec{w}^{-1}\vec{u}^{\prime}\vec{w}$$
$$ \vec{u}^{\prime\prime} = (\vec{w}^{-1}\vec{v}^{-1})\vec{u}(\vec{v}\vec{w})$$

The parenthesis are not necessary here but I added them for clarity.

Now because we can freely choose $\vec{v}$ and $\vec{w}$, we set them to a length of 1. And remember that the inverse is defined as:

$$ \vec{v}^{-1} = \frac{\vec{v}}{|\vec{v}|^2}$$
if $ |\vec{v}| = 1$ and $ |\vec{w}| = 1$ then we get
$$ \vec{v}^{-1} = \vec{v}$$
$$ \vec{w}^{-1} = \vec{w}$$

Now we can rewrite the equation from above

$$ \vec{u}^{\prime\prime} = (\vec{w}\vec{v})\vec{u}(\vec{v}\vec{w})$$

Also remember that a bivector on the left has the opposite rotation as a bivector on the right and if we swap the order of the bivector we also get the opposite rotation. That means that we can do the following.

$$ \vec{u}^{\prime\prime} = \vec{u}(-\vec{w}\vec{v}\vec{v}\vec{w})$$
$$ \vec{u}^{\prime\prime} = \vec{u}(\vec{v}\vec{w}\vec{v}\vec{w})$$
$$ \vec{u}^{\prime\prime} = \vec{u}(\vec{v}\vec{w})^2$$


We can see that a double reflection is the same applying the geometric product of $\vec{v}\vec{w}$ two times. We also see that with a double reflection we can rotate a vector around any angle that we would like.

The important angle is the angle between $\vec{v}$ and $\vec{w}$. In the case above the angle was $90\degree$, but we did a rotation of $180\degree$. It is easy to see why this happens.

Let us assume that:

$ \alpha = $ angle between $\vec{u}$ and $\vec{w} $

$ \beta = $ angle between $\vec{u}$ and $\vec{v} $

$ \theta = $ angle of the full rotation 

$$ \theta = 2\beta + 2(\alpha - \beta) $$

Where $2\beta$ is the rotation from the reflection of $\vec{u}$ and $\vec{v}$. And the angle between $\vec{w}$ and $\vec{u}^{\prime}$ is $\alpha - \beta$.
$$ \theta = 2\alpha$$

Or

$$ \alpha = \frac{\theta}{2}$$

Now we can finally rotate a vector with an arbitrary angle. We only need to find two vectors that are not linearly dependent and 

Example:

$$ \vec{v} = \vec{x} $$

$$ \vec{w} = \frac{1}{\sqrt{2}}(\vec{x} + \vec{y}) $$

$$ \vec{u} = \vec{x} $$

The angle between $\vec{v}$ and $\vec{w}$ is $45\degree$, and both are unit vectors.
$$ \vec{u}(\vec{v}\vec{w})^2 = \vec{x}(\vec{x}\frac{1}{\sqrt{2}}(\vec{x} + \vec{y}))^2$$
$$= \vec{x}(\frac{1}{\sqrt{2}}\vec{x}\vec{x} + \frac{1}{\sqrt{2}}\vec{x}\vec{y})^2 $$
$$= \vec{x}(\frac{1}{\sqrt{2}} + \frac{1}{\sqrt{2}}\vec{x}\vec{y})^2$$
$$= \vec{x}(\frac{1}{\sqrt{2}} + \frac{1}{\sqrt{2}}\vec{x}\vec{y})(\frac{1}{\sqrt{2}} + \frac{1}{\sqrt{2}}\vec{x}\vec{y})$$
$$ = (\frac{1}{\sqrt{2}}\vec{x} + \frac{1}{\sqrt{2}}\vec{x}\vec{x}\vec{y})(\frac{1}{\sqrt{2}} + \frac{1}{\sqrt{2}}\vec{x}\vec{y}) $$
$$ = (\frac{1}{\sqrt{2}}\vec{x} + \frac{1}{\sqrt{2}}\vec{y})(\frac{1}{\sqrt{2}} + \frac{1}{\sqrt{2}}\vec{x}\vec{y}) $$
$$ = \frac{1}{2}\vec{x} + \frac{1}{2}\vec{x}\vec{x}\vec{y} + \frac{1}{2}\vec{x}\vec{x}\vec{y} + \frac{1}{2}\vec{x}\vec{x}\vec{y}\vec{x}\vec{y}$$
$$ = \frac{1}{2}\vec{x} + \frac{1}{2}\vec{y} + \frac{1}{2}\vec{y} + \frac{1}{2}\vec{y}\vec{x}\vec{y}$$
$$ = \frac{1}{2}\vec{x} + \frac{1}{2}\vec{y} + \frac{1}{2}\vec{y} - \frac{1}{2}\vec{x}\vec{y}\vec{y}$$
$$ = \frac{1}{2}\vec{x} + \frac{1}{2}\vec{y} + \frac{1}{2}\vec{y} - \frac{1}{2}\vec{x}$$
$$ = \frac{1}{2}\vec{y} + \frac{1}{2}\vec{y}$$
$$ = \vec{y}$$

We rotated $\vec{u}$ from $\vec{x}$ to $\vec{y}$.

## Rotator

$$\vec{u}\vec{v} = \vec{u} \cdot \vec{v} + \vec{u} \wedge \vec{v} $$

There is another interesting property.
$$\vec{u}\vec{v} = |\vec{u}||\vec{v}|\cos{\theta} + |\vec{u}||\vec{v}|\sin{\theta}\vec{x} \wedge \vec{y} $$
$$\vec{u}\vec{v} = |\vec{u}||\vec{v}|\cos{\theta} + |\vec{u}||\vec{v}|\sin{\theta}I $$

Where $I=$ some bivector
$$\vec{u}\vec{v} = |\vec{u}||\vec{v}|(\cos{\theta} + \sin{\theta}I) $$
$$\vec{u}\vec{v} = |\vec{u}||\vec{v}|e^{\theta I} $$

We can derive another inverse

$$\vec{u}\vec{v}x = |\vec{u}||\vec{v}|e^{\theta I}x = 1 $$

We are looking for an $x$ that satisfies this equation. The first thing we are going to do is to divide by the scalar $\frac{1}{|\vec{u}||\vec{v}|}$ and now we need to revert the rotation
which is just $e^{-\theta I}$

$$ (\vec{u}\vec{v})^{-1} = \frac{1}{|\vec{u}||\vec{v}|}e^{-\theta I}$$

Before we can move on we need to prove


$$ (\vec{u}\vec{v})^{-1} = \vec{u}^{-1}\vec{v}^{-1} = \frac{\vec{u}}{|\vec{u}|}\frac{\vec{v}}{|\vec{v}|} = \frac{1}{|\vec{u}||\vec{v}|}\vec{u}\vec{v}$$ 
