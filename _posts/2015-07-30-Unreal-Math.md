---
layout: post
title: "Gamedev Math: Vector projection"
description: ""
category:
tags: []
---
Today we will learn how we can project a vector onto another vector.

Vector projection is defined in UE4 like this:

{% highlight c++ %}
FORCEINLINE FVector FVector::ProjectOnTo( const FVector& A ) const 
{ 
	return (A * ((*this | A) / (A | A))); 
}
{% endhighlight %}

Depending on your familiarity with math or UE4 this can look at bit strange to you.

The | operator in UE4 is defined as the [dot product](https://en.wikipedia.org/wiki/Dot_product).

Also there exists an interesting equality for the dot product, which you can get by using the [Law of Cosines](https://en.wikipedia.org/wiki/Law_of_cosines).

$$\vec{a} {\cdot} \vec{b}= ||\vec{a}|| * ||\vec{b}|| * cos(\alpha)$$

The notation \\( ||\vec{v}|| \\) is known as the [length of a vector](https://en.wikipedia.org/wiki/Euclidean_vector#Length).



![proj]({{ site.url }}/assets/pictures/math-vector-projection.png)


We want to project \\( \vec{a}\\) onto \\( \vec{b}\\). The yellow \\( \vec{p}\\) is the vector that we are interested in and as you can see they form a right angled triangle.

First let me show you a regular triangle


![triangle]({{ site.url }}/assets/pictures/triangle.png)


$$ cos(\alpha) = \dfrac{adjacent}{hypotenuse}$$

or

$$ cos(\alpha) = \dfrac{b}{a}$$

If we know \\(a\\) and \\(cos(\alpha)\\) we can easily compute the value of \\(b\\)

$$ cos(\alpha) * a = b $$

Let's get back to the dot product

$$\vec{a} {\cdot} \vec{b}= ||\vec{a}|| * ||\vec{b}|| * cos(\alpha)$$

which is the same thing as

$$cos(\alpha) * ||\vec{a}|| = \dfrac{\vec{a} {\cdot} \vec{b}} {||\vec{b}||}$$

Now remember that we can compute \\(b\\) in a triangle 

$$ cos(\alpha) * a = b $$

which means that we can use $$\dfrac{\vec{a} {\cdot} \vec{b}} {||\vec{b}||} = ||\vec{p}|| $$ to compute the length of our projected vector \\( \vec{p} \\).

By [normalizing](https://en.wikipedia.org/wiki/Unit_vector) \\( \vec{b}\\) and multiplying it by our computed length we get \\( \vec{p} \\).

$$ \vec{p} = \dfrac{\vec{b}}{||\vec{b}||} * \dfrac{\vec{a} {\cdot} \vec{b}} {||\vec{b}||} =  \vec{b}* \dfrac{\vec{a} {\cdot} \vec{b}} {||\vec{b}||^2} =  \vec{b}* \dfrac{\vec{a} {\cdot} \vec{b}} {\vec{b} {\cdot} {\vec{b}} }$$

{% highlight c++ %}
FORCEINLINE FVector FVector::ProjectOnTo( const FVector& A ) const 
{ 
	return (A * ((*this | A) / (A | A))); 
}
{% endhighlight %}

I hope that you understand know how this function works.





