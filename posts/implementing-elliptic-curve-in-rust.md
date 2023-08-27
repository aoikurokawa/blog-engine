# Implementing Elliptic Curves in Rust: A Comprehensive Guide

## Introduction

In the [last blog](https://www.nxted.co.jp/blog/blog_detail?id=21), we learned about finite field and implemented it in Rust programming language.   
In this blog post, we are going to learn Elliptic Curves. Like last blog, I will explain the defintion of elliptic curve and will implement in Rust programming language. 
Let's get started!


## Understanding Elliptic Curve

An elliptic curve is a mathematical concept that plays a crucial role in various fields, including cryptography and number theory. It is a specific type of curve defined by an equation of the form:
y² = x³ + ax + b

Here, "a" and "b" are constants that define the characteristics of the curve. The curve is plotted on a two-dimensional plane with an x-axis and a y-axis. The interesting and useful property of elliptic curves is their ability to form a group structure with a particular point called the "point at infinity" acting as the identity element.

Elliptic curves have several intriguing properties:

- Symmetry: An elliptic curve is symmetric with respect to the x-axis, meaning that if a point (x, y) lies on the curve, the point (x, -y) also lies on the curve.

- Cubic Nature: The equation of an elliptic curve involves cubic terms, which give the curve its distinct shape.

- Multiple Intersections: An elliptic curve can intersect the x-axis at one, two, or three points, including the point at infinity. This property is used in cryptographic applications.

- Group Structure: The set of points on an elliptic curve, along with an operation called "point addition," forms an additive abelian group. This property is fundamental in cryptographic applications, particularly in elliptic curve cryptography (ECC).

Elliptic curve cryptography (ECC) leverages the difficulty of solving certain mathematical problems involving elliptic curves for its security. One of the most well-known uses of ECC is in public key cryptography, where the security of communication relies on the difficulty of solving the elliptic curve discrete logarithm problem. For more detail, I will try to explain in the next blog. 


## Coding Elliptic Curves in Rust

### Define a Point struct

```rust
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub a: i32,
    pub b: i32,
}
```

### Implement two methods: `new` and `equal`

```rust
impl Point {
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Self {
        if y.pow(2) != x.pow(3) + (a * x) + b {
            panic!("({}, {}) is not on the curve", x, y);
        }

        Self { a, b, x, y }
    }

    pub fn equal(&self, other: Option<Point>) -> bool {
        *self == other.unwrap()
    }
}
```

Let's check equal method is working or not. 

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_equal() {
        let point1 = Point::new(-1, -1, 5, 7);
        let point2 = Point::new(-1, -1, 5, 7);

        assert!(point1.equal(Some(point2)));
  }
}
```

### Point Addition

```rust


```


## Conclusion
Thank you for reading. See you in next blog. 
