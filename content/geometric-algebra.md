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

But we are limited to $90\degree$ rotations.

## Inverse

$$ \vec{u}\vec{u} = \vec{u} \cdot \vec{u} = |\vec{u}|^2$$
$$ \vec{u}\vec{u} = |\vec{u}|^2 $$
$$ \frac{\vec{u}}{|\vec{u}|^2}\vec{u} = 1 $$
$$ \vec{u}^{-1} = \frac{\vec{u}}{|\vec{u}|^2}$$
$$ \vec{u}^{-1}\vec{u} = \vec{u}\vec{u}^{-1} = 1$$

## Reflection

Before we can learn how to do rotations, we must learn how to do reflections.
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

Example:

$$\vec{u} = 3\vec{x} + \vec{y}$$
$$\vec{v} = 2\vec{x} + 2\vec{y}$$

$$\vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v} = \frac{2\vec{x} + 2\vec{y}}{ 2 * 2 + 2 * 2}(3\vec{x} + \vec{y})(2\vec{x} + 2\vec{y})$$
$$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(3\vec{x} + \vec{y})(2\vec{x} + 2\vec{y})$$
$$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(3\vec{x}2\vec{x} + 3\vec{x}2\vec{y} + \vec{y}2\vec{x} + \vec{y}2\vec{y})$$
$$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(6\vec{x}\vec{x} + 6\vec{x}\vec{y} + 2\vec{y}\vec{x} + 2\vec{y}\vec{y})$$
$$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(8 + 4\vec{x}\vec{y})$$
$$= 8\frac{1}{4}\vec{x} +\frac{1}{4}\vec{x}4\vec{x}\vec{y} + 8\frac{1}{4}\vec{y} + \frac{1}{4}\vec{y}4\vec{x}\vec{y}$$
$$= 2\vec{x} +\vec{x}\vec{x}\vec{y} + 2\vec{y} + \vec{y}\vec{x}\vec{y}$$
$$= 2\vec{x} +\vec{y} + 2\vec{y} - \vec{x}\vec{y}\vec{y}$$
$$= 2\vec{x} +\vec{y} + 2\vec{y} - \vec{x}$$
$$= \vec{x} + 3\vec{y}$$
## Double reflection as a rotation
![](https://i.imgur.com/Kzp6tzy.png)

Instead of reflecting $\vec{u}$ along $\vec{v}$, we will introduce another vector $\vec{w}$ and do a double reflection.

To get the first reflection we use the equation that we derived above.


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

Now we can finally rotate a vector with an arbitrary angle. If we want to do a $90\degree$ rotation, then the angle between $\vec{v}$ and $\vec{w}$ should be $45\degree$. This property is called double cover.

## 3D

All the examples that we looked at so far were all in 2D, you might expect more complexity if we want to do rotation in 3D, but that is not the case. Let us do a 3D rotation.
![](https://i.imgur.com/Z2JCvwh.png)

$$\vec{u} = \vec{x} + \vec{y} + \vec{z}$$

We are going to make a $180\degree$ rotation in the $\vec{x}\vec{y}$ plane. That means that the angle between our vectors should be $90\degree$.

As before we will do a 3D rotation by double reflection.
$$ \vec{u^{\prime}} = (\vec{y}\vec{x})\vec{u}(\vec{x}\vec{y}) $$

$$ = (\vec{y}\vec{x})(\vec{x} + \vec{y} + \vec{z})(\vec{x}\vec{y}) $$
$$ = (\vec{y}\vec{x})(\vec{x} + \vec{y} + \vec{z})(\vec{x}\vec{y}) $$
$$ = \vec{y}\vec{x}\vec{x}\vec{x}\vec{y} +\vec{y}\vec{x}\vec{y}\vec{x}\vec{y}+\vec{y}\vec{x}\vec{z}\vec{x}\vec{y} $$
$$ = \vec{y}\vec{x}\vec{y}  - \vec{y}\vec{y}\vec{x}\vec{x}\vec{y}-\vec{y}\vec{x}\vec{x}\vec{z}\vec{y} $$
$$ = -\vec{y}\vec{y}\vec{x}  - \vec{y}-\vec{y}\vec{z}\vec{y} $$
$$ = -\vec{x}  - \vec{y} + \vec{z}\vec{y}\vec{y} $$
$$ = -\vec{x}  - \vec{y} + \vec{z} $$

And we get the correct result. But why can we use double reflection in 3D?

We choose two vectors that span a plane in 3D space, we call them $\vec{v}$ and $\vec{w}$. 

$$|\vec{w}| = |\vec{w}| = 1$$

$$ R = \vec{v}\vec{w} $$
$$ R^{\dag} = \vec{w}\vec{v} $$


$$ \vec{u} = \vec{u}\_{\parallel} + \vec{u}\_{\perp} $$

Where $\vec{u}\_{\perp}$ is perpendicular to $\vec{v}$ and $\vec{w}$ and \vec{u}\_{\parallel} is parallel to $\vec{v}$ and $\vec{w}$.

## Quaternions
$$\vec{u} \cdot \vec{v} = |\vec{u}||\vec{v}|\cos{\alpha}$$

![](https://i.imgur.com/yxZyLdA.png)

Remember that $\vec{u} \wedge \vec{v}$ gives us a unit bivector with the signed area that $\vec{u}$ and $\vec{v}$ span. The area of a parallelogram is base $*$ height.

$$\vec{u} \wedge \vec{v} = |\vec{u}||\vec{v}|\sin{\alpha}\vec{x}\vec{y}\vec{z}$$

$$\vec{u}\vec{v} = |\vec{u}||\vec{v}|\cos{\alpha} + |\vec{u}||\vec{v}|\sin{\alpha}\vec{x}\vec{y}\vec{z} $$

And if $|\vec{u}| = |\vec{v}| = 1$ then
$$\vec{u}\vec{v} = \cos{\alpha} + \sin{\alpha}\vec{x}\vec{y}\vec{z} $$
And the inverse
$$\vec{v}\vec{u} = \cos{\alpha} - \sin{\alpha}\vec{x}\vec{y}\vec{z} $$

The dot product doesn't change because it is commutative, but the order of the wedge product switched and therefor switches the sign.
$$\vec{u} \wedge \vec{v} = -(\vec{v} \wedge \vec{u})$$

But remember that we are rotation by double reflection and $ \alpha = \frac{\theta}{2}$.

$$\vec{u}\vec{v} = \cos{\frac{\theta}{2}} + \sin{\frac{\theta}{2}}\vec{x}\vec{y}\vec{z} $$
$$\vec{v}\vec{u} = \cos{\frac{\theta}{2}} - \sin{\frac{\theta}{2}}\vec{x}\vec{y}\vec{z} $$
