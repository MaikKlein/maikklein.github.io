+++
title= "Fantastic quaternions and where to find them"
date        = "2017-12-30"
+++

# Wedge product
The wedge product will give us there area that two vectors are spanning.

We start of with two properties:

$$ \vec{u} \wedge \vec{v} = -(\vec{u} \wedge \vec{v}) $$

The wedge product will give us the signed area of the parallelogram, just like the determinant.

$$ \vec{u} \wedge \vec{u} = 0$$

And if two vectors linearly dependent, they will have an area of 0. With these two properties, we can derive the actual formula to calculate the area that two vectors are spanning.

$$ \vec{u} = a\vec{x} + b\vec{y}$$

$$ \vec{v} = c\vec{x} + d\vec{y}$$

$$ \vec{u} \wedge \vec{v} = (a\vec{x} + b\vec{y}) \wedge (c\vec{x} + d\vec{y}) $$ $$ = a\vec{x} \wedge c\vec{x} + a\vec{x} \wedge d\vec{y} + b\vec{y} \wedge cx + b\vec{y} \wedge d\vec{y}$$ $$ = a\vec{x} \wedge d\vec{y} + b\vec{y} \wedge c\vec{x}$$ $$ = a\vec{x} \wedge d\vec{y} - c\vec{x} \wedge b\vec{y}$$ $$ = ad (\vec{x} \wedge \vec{y}) - bc(\vec{x} \wedge \vec{y})$$ $$ = (ad - bc) \vec{x} \wedge \vec{y}$$ This is essentially the 2d determinant times $\vec{x} \wedge \vec{y} $. The area of $\vec{x} \wedge \vec{y} $ is 1. From now own we will call this a bivector. A bivector therefor consists of two vectors that form a plane, and its magnitude is the signed area that those vectors are spanning.
# Geometric product
$$\vec{u}\vec{v} = \vec{u} \cdot \vec{v} + \vec{u} \wedge \vec{v} $$

The geometric product is just the dot product + the wedge product.

The geometric product is both commutative and anti-commutative.

If $\vec{u}$ and $\vec{v}$ are orthogonal, then the dot product becomes 0 and we are left with the wedge product $$ \vec{u}\vec{v} = \vec{u} \wedge \vec{v}$$ and therefor is anti-commutative $$ \vec{u}\vec{v} = -\vec{v}\vec{u}$$

If $\vec{u}$ and $\vec{v}$ are linearly dependent, then the wedge product becomes 0 and we are left with only the dot product. $$ \vec{u}\vec{v} = \vec{u} \cdot \vec{v}$$ $$ \vec{u}\vec{v} = \vec{v}\vec{u}$$

You can also write $ \vec{x}\vec{x} $ as

$$ \vec{x}\vec{x} = \vec{x}^2 $$ $$ \vec{x}^2 = \vec{x}\vec{x} = \vec{x} \cdot \vec{x} = 1 $$

Now we will look at the geometric product of $\vec{x}\vec{y} $. $$ \vec{x}\vec{y} = \vec{x} \wedge \vec{y}$$

Because of this we get

$$ \vec{x}\vec{y} = -(\vec{y}\vec{x})$$ $$ (\vec{x}\vec{y})^2 = \vec{x}\vec{y}\vec{x}\vec{y}$$

Remember order matters.

$$ (\vec{x}\vec{y})^2 = \vec{x}(-\vec{x}\vec{y})\vec{y} = -\vec{x}\vec{x}\vec{y}\vec{y} = -\vec{x}^2\vec{y}^2 = -1$$

Interestingly the squared geometric product of $\vec{x}\vec{y}$ will result in $-1$. This is very similar to $i^2 = -1$. You might wonder if we can do rotations with $\vec{x}\vec{y}$ and the answer is yes we can.

$$ \vec{u} = a\vec{x} + b\vec{y} $$ $$ \vec{u}\vec{x}\vec{y} = (a\vec{x} + b\vec{y})\vec{x}\vec{y} = a\vec{x}\vec{x}\vec{y} + b\vec{y}\vec{x}\vec{y} = a\vec{x}\vec{x}\vec{y} - b\vec{x}\vec{y}\vec{y} = - b\vec{x} + a\vec{y} $$

You can compare this to

$$ (a + bi)i = ai + bi^2 = -b + ai $$

# Inverse

$$ \vec{u}\vec{u} = \vec{u} \cdot \vec{u} = |\vec{u}|^2$$
$$ \vec{u}\vec{u} = |\vec{u}|^2 $$
$$ \frac{\vec{u}}{|\vec{u}|^2}\vec{u} = 1 $$
$$ \vec{u}^{-1} = \frac{\vec{u}}{|\vec{u}|^2}$$
$$ \vec{u}^{-1}\vec{u} = \vec{u}\vec{u}^{-1} = 1$$

# Reflection

Before we can learn how to do rotations, we must learn how to do reflections.
![](https://i.imgur.com/drCMAoR.png)
$$ \vec{u} = \vec{u}_\perp + \vec{u}_{\parallel} $$ $$ \vec{u}^{\prime} =\vec{u}_{\parallel} - \vec{u}_\perp$$ We are going to define vector reflection with the geometric product. $$ \vec{u}^{\prime}\vec{v}\ = \vec{u}_{\parallel}\vec{v}\ - \vec{u}_\perp\vec{v}$$ $$ \vec{u}^{\prime}\vec{v}\ = \vec{v}\vec{u}_{\parallel} + \vec{v}\vec{u}_\perp$$ $$ \vec{u}^{\prime}\vec{v}\ = \vec{v}\vec{u}_{\parallel} + \vec{v}\vec{u}_\perp$$ $$ \vec{u}^{\prime}\vec{v}\ = \vec{v}(\vec{u}_{\parallel} + \vec{u}_\perp)$$ $$ \vec{u}^{\prime}\vec{v}\ = \vec{v}\vec{u}$$ $$ \vec{u}^{\prime}\vec{v}\vec{v}^{-1} = \vec{v}\vec{u}\vec{v}^{-1}$$ $$ \vec{u}^{\prime} = \vec{v}\vec{u}\vec{v}^{-1}$$ Interestingly we could have also used the opposite approach. $$ \vec{u} = \vec{u}_\perp + \vec{u}_{\parallel} $$ $$ \vec{u}^{\prime} =\vec{u}_{\parallel} - \vec{u}_\perp$$ $$ \vec{v}\vec{u}^{\prime} = \vec{v}\vec{u}_{\parallel} - \vec{v}\vec{u}_\perp$$ $$ \vec{v}\vec{u}^{\prime} = \vec{u}_{\parallel}\vec{v} + \vec{u}_\perp\vec{v}$$ $$ \vec{v}\vec{u}^{\prime} = (\vec{u}_{\parallel} + \vec{u}_\perp)\vec{v}$$ $$ \vec{v}\vec{u}^{\prime} = \vec{u}\vec{v}$$ $$ \vec{v}^{-1}\vec{v}\vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v}$$ $$ \vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v}$$

Example:

$$\vec{u} = 3\vec{x} + \vec{y}$$ $$\vec{v} = 2\vec{x} + 2\vec{y}$$

$$\vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v} = \frac{2\vec{x} + 2\vec{y}}{ 2 * 2 + 2 * 2}(3\vec{x} + \vec{y})(2\vec{x} + 2\vec{y})$$ $$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(3\vec{x} + \vec{y})(2\vec{x} + 2\vec{y})$$ $$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(3\vec{x}2\vec{x} + 3\vec{x}2\vec{y} + \vec{y}2\vec{x} + \vec{y}2\vec{y})$$ $$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(6\vec{x}\vec{x} + 6\vec{x}\vec{y} + 2\vec{y}\vec{x} + 2\vec{y}\vec{y})$$ $$= (\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})(8 + 4\vec{x}\vec{y})$$ $$= 8\frac{1}{4}\vec{x} +\frac{1}{4}\vec{x}4\vec{x}\vec{y} + 8\frac{1}{4}\vec{y} + \frac{1}{4}\vec{y}4\vec{x}\vec{y}$$ $$= 2\vec{x} +\vec{x}\vec{x}\vec{y} + 2\vec{y} + \vec{y}\vec{x}\vec{y}$$ $$= 2\vec{x} +\vec{y} + 2\vec{y} - \vec{x}\vec{y}\vec{y}$$ $$= 2\vec{x} +\vec{y} + 2\vec{y} - \vec{x}$$ $$= \vec{x} + 3\vec{y}$$

$$\vec{u}^{\prime} = \vec{v}\vec{u}\vec{v}^{-1} = (2\vec{x} + 2\vec{y})(3\vec{x} + \vec{y})(\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})$$ $$ = (8 - 4\vec{x}\vec{y})(\frac{1}{4}\vec{x} + \frac{1}{4}\vec{y})$$ $$ = 2x + 2y - xyx -xyy $$ $$ = 2x + 2y + y -x $$ $$ = x + 3y$$
# Double reflection as a rotation
![](https://i.imgur.com/Kzp6tzy.png)
Instead of reflecting $\vec{u}$ along $\vec{v}$, we will introduce another vector $\vec{w}$ and do a double reflection.

To get the first reflection we use the equation that we derived above.

$$ \vec{u}^{\prime} = \vec{v}^{-1}\vec{u}\vec{v}$$ $$ \vec{u}^{\prime\prime} = \vec{w}^{-1}\vec{u}^{\prime}\vec{w}$$ $$ \vec{u}^{\prime\prime} = (\vec{w}^{-1}\vec{v}^{-1})\vec{u}(\vec{v}\vec{w})$$

The parenthesis are not necessary here but I added them for clarity.

Now because we can freely choose $\vec{v}$ and $\vec{w}$, we set them to a length of 1. And remember that the inverse is defined as:

$$ \vec{v}^{-1} = \frac{\vec{v}}{|\vec{v}|^2}$$ if $ |\vec{v}| = 1$ and $ |\vec{w}| = 1$ then we get $$ \vec{v}^{-1} = \vec{v}$$ $$ \vec{w}^{-1} = \vec{w}$$

Now we can rewrite the equation from above

$$ \vec{u}^{\prime\prime} = (\vec{w}\vec{v})\vec{u}(\vec{v}\vec{w})$$

In geometric algebra, this is called a rotor $R$.

$$R = \vec{v}\vec{w}$$ $$R^{\dag} = \vec{w}\vec{v}$$

$$ \vec{u^{\prime}} = R\vec{u}R^{\dag} = R^{\dag}\vec{u}R $$ The important angle is the angle between $\vec{v}$ and $\vec{w}$. In the case above the angle was $90\degree$, but we did a rotation of $180\degree$. It is easy to see why this happens.

Let us assume that:

$ \alpha = $ angle between $\vec{v}$ and $\vec{w} $

$ \beta = $ angle between $\vec{u}$ and $\vec{v} $

$ \theta = $ angle of the full rotation

$$ \theta = 2\beta + 2(\alpha - \beta) $$

Where $2\beta$ is the angle from the reflection of $\vec{u}$ and $\vec{v}$. And the angle between $\vec{w}$ and $\vec{u}^{\prime}$ is $\alpha - \beta$. $$ \theta = 2\alpha$$

Or

$$ \alpha = \frac{\theta}{2}$$

We can finally rotate a vector with an arbitrary angle. If we want to rotate around $\alpha$ we do a double reflection, where the angle between $\vec{v}$ and $\vec{w}$ is $\frac{\alpha}{2}$.
# 3D

All the examples that we looked at so far were all in 2D, you might expect more complexity if we want to do rotation in 3D, but that is not the case. Let us do a 3D rotation.
![](https://i.imgur.com/Z2JCvwh.png)

We are going to make a $180\degree$ rotation in the $\vec{x}\vec{y}$ plane. That means that the angle between our vectors should be $90\degree$. We can choose any two unit vectors in the $\vec{x}\vec{y}$ plane but for simplicity I will choose the basis vectors $\vec{x}\vec{y}$.

As before we will do a 3D rotation by double reflection. $$\vec{u} = \vec{x} + \vec{y} + \vec{z}$$ $$ R = \vec{x}\vec{y}$$ $$ R^{\dag} = \vec{y}\vec{x}$$

$$\vec{u^{\prime}} = R^{\dag}\vec{u}R = (\vec{y}\vec{x})(\vec{x} + \vec{y} + \vec{z})(\vec{x}\vec{y}) $$ $$ = \vec{y}\vec{x}\vec{x}\vec{x}\vec{y} +\vec{y}\vec{x}\vec{y}\vec{x}\vec{y}+\vec{y}\vec{x}\vec{z}\vec{x}\vec{y} $$ $$ = \vec{y}\vec{x}\vec{y} - \vec{y}\vec{y}\vec{x}\vec{x}\vec{y}-\vec{y}\vec{x}\vec{x}\vec{z}\vec{y} $$ $$ = -\vec{y}\vec{y}\vec{x} - \vec{y}-\vec{y}\vec{z}\vec{y} $$ $$ = -\vec{x} - \vec{y} + \vec{z}\vec{y}\vec{y} $$ $$ = -\vec{x} - \vec{y} + \vec{z} $$

And we get the correct result. But why can we do double reflection in 3D?

Let us choose two vectors that span a plane in 3D space, we call them $\vec{v}$ and $\vec{w}$.

$$|\vec{w}| = |\vec{w}| = 1$$

$$ R = \vec{v}\vec{w} $$ $$ R^{\dag} = \vec{w}\vec{v} $$

$$ \vec{u} = \vec{u}_{\parallel} + \vec{u}_{\perp} $$

Where $\vec{u}_{\perp}$ is perpendicular to $\vec{v}$ and $\vec{w}$ and $\vec{u}_{\parallel}$ is parallel to $\vec{v}$ and $\vec{w}$.

$$ \vec{u^{\prime}} = R^{\dag}\vec{u}R = (\vec{w}\vec{v})(\vec{u}_{\parallel} + \vec{u}_{\perp})(\vec{v}\vec{w}) $$ $$=\vec{w}\vec{v}\vec{u}_{\parallel}\vec{v}\vec{w} + \vec{w}\vec{v}\vec{u}_{\perp}\vec{v}\vec{w}$$

Let us pause here for a second. We have $\vec{w}\vec{v}\vec{u}_{\perp}\vec{v}\vec{w}$ and remember that $\vec{u}_{\perp}$ is perpendicular to $\vec{v}$ and $\vec{w}$.

Also remember that if two vectors are perpendicular the dot product is 0 and we are left with only the wedge product.

$$ \vec{w}\vec{v}\vec{u}_{\perp}\vec{v}\vec{w} = -\vec{w}\vec{u}_{\perp}\vec{v}\vec{v}\vec{w} = \vec{u}_{\perp}\vec{w}\vec{v}\vec{v}\vec{w} = \vec{u}_{\perp}\vec{w}\vec{w} = \vec{u}_{\perp} $$

$$\vec{u^{\prime}} = \vec{w}\vec{v}\vec{u}_{\parallel}\vec{v}\vec{w} + \vec{u}_{\perp}$$

We only rotate the part of $\vec{u}$ that is parallel to $\vec{v}$ and $\vec{w}$. This is exactly how we did double reflection in 2D.

Previously we have seen that there are two ways to do rotations

$$ \vec{u^{\prime}} = R\vec{u}R^{\dag} = R^{\dag}\vec{u}R $$

We expect that if we apply the same rotation twice, to get double the rotation.

$$|\vec{w}| = |\vec{w}| = 1$$ $$ R = \vec{v}\vec{w} $$ $$ R^{\dag} = \vec{w}\vec{v} $$ $$ \vec{u^{\prime}} = R\vec{u}R^{\dag} $$ $$ \vec{u^{\prime\prime}} = R^{\dag}\vec{u^{\prime}}R $$ $$ \vec{u^{\prime\prime}} = R^{\dag}(R\vec{u}R^{\dag})R $$ $$ \vec{u^{\prime\prime}} = \vec{w}\vec{v}\vec{v}\vec{w}\vec{u}\vec{w}\vec{v}\vec{v}\vec{w} $$ $$ \vec{u^{\prime\prime}} = \vec{w}\vec{w}\vec{u}\vec{w}\vec{w} $$ $$ \vec{u^{\prime\prime}} = \vec{u} $$

Something strange is happening here. We know that there are two ways of doing the same rotation $ \vec{u^{\prime}} = R\vec{u}R^{\dag} = R^{\dag}\vec{u}R $, but if we rotate first with $R\vec{u}R^{\dag}$ and then again with $R^{\dag}\vec{u^{\prime}}R$ we get back to our original vector $\vec{u}$. This happens because we actually didn't apply the rotation two times, but we combined the rotors $\vec{u^{\prime\prime}} = (R^{\dag}R)\vec{u}(R^{\dag}R) $. Because $R^{\dag}$ is the inverse of $R$, they will just cancel.

To correctly combine rotors, you need to use the same formula for reflection. $$\vec{u^{\prime\prime}} = (RR)\vec{u}(R^{\dag}R^{\dag}) =(R^{\dag}R^{\dag})\vec{u}(RR) $$
# Quaternions
$$ \vec{v^{\prime}} = q\vec{v}q^{-1} = (\cos{\frac{\alpha}{2}} + \vec{u}\sin{\frac{\alpha}{2}})\vec{v}(\cos{\frac{\alpha}{2}} - \vec{u}\sin{\frac{\alpha}{2}})$$

Notice that $q\vec{v}q^{-1}$ and $\frac{\alpha}{2}$ looks like a double reflection, but where are the $\cos$ and $\sin$ coming from?

The geometric product is defined as the dot product + the wedge product. $$\vec{u}\vec{v} = \vec{u} \cdot \vec{v} + \vec{u} \wedge \vec{v} $$

One property of the dot product is $\vec{u} \cdot \vec{v} = |\vec{u}||\vec{v}|\cos{\alpha}$.
![](https://i.imgur.com/yxZyLdA.png)
Remember that $\vec{u} \wedge \vec{v}$ gives us a unit bivector with the signed area that $\vec{u}$ and $\vec{v}$ span. The area of a parallelogram is base $*$ height.

$$\vec{u} \wedge \vec{v} = |\vec{u}||\vec{v}|\sin({\alpha})B$$

$B$ is some unit bivector.

$$\vec{u}\vec{v} = |\vec{u}||\vec{v}|\cos({\alpha}) + |\vec{u}||\vec{v}|\sin({\alpha})B $$

And if $|\vec{u}| = |\vec{v}| = 1$ then $$\vec{u}\vec{v} = \cos({\alpha}) + \sin({\alpha})B $$ And the inverse $$\vec{v}\vec{u} = \cos({\alpha}) - \sin({\alpha})B $$

The sign of dot product doesn't change because it is commutative, but the order of the wedge product switches and therefor changes the sign, which gives us $-\sin$ for the inverse.

But remember that we are rotating by double reflection and $ \alpha = \frac{\theta}{2}$.

$$\vec{u}\vec{v} = \cos({\frac{\theta}{2}}) + \sin({\frac{\theta}{2}})B $$ $$\vec{v}\vec{u} = \cos({\frac{\theta}{2}}) - \sin({\frac{\theta}{2}})B $$

$$ \vec{v^{\prime}} = R\vec{u}R^{\dag} = (\cos({\frac{\alpha}{2}}) + \sin({\frac{\alpha}{2}})B)\vec{v}(\cos({\frac{\alpha}{2}}) - \sin({\frac{\alpha}{2}})B)$$

This looks very similar to a quaternion.

$$ \vec{v^{\prime}} = q\vec{v}q^{-1} = (\cos{\frac{\alpha}{2}} + \vec{u}\sin{\frac{\alpha}{2}})\vec{v}(\cos{\frac{\alpha}{2}} - \vec{u}\sin{\frac{\alpha}{2}})$$

# Double cover
Let $\alpha = 2\pi$

$$ R = \vec{u}\vec{v} = \cos({\frac{\alpha}{2}}) + \sin({\frac{\alpha}{2}})B) $$

$$ R = \cos{\pi} + \sin({\pi})B = -1 $$

Let $\alpha = 4\pi$

$$ R = \cos({2\pi}) + \sin({2\pi})B = 1 $$ A $2\pi$ rotation is the same as a $4\pi$ rotation but we end up with two different rotors. $$ \vec{u^{\prime}} = R\vec{u}R^{\dag} = (\vec{v}\vec{w})\vec{u}(\vec{w}\vec{v})$$ $$ = --(\vec{v}\vec{w})\vec{u}(\vec{w}\vec{v})$$ $$ = (-\vec{v}\vec{w})\vec{u}(-\vec{w}\vec{v})$$

There are two different rotors that will apply the same rotation. This property is called double cover.

# Recap
We started with the definition of the geometric product, then we learned that we can express any rotation by double reflection. We derived a formula for double reflection by using the geometric product which we called rotors. After that we compared rotors to quaternions and found that they look almost identical.
