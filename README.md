# CATAM-9.4-Option-Pricing-in-Mathematical-Finance
A rewrite of my Cambridge CATAM undergraduate project, "Option Pricing in Mathematical Finance"

## Rust

This project is written in [Rust](https://www.rust-lang.org/), which is a highly performant compiled language with memory management. Rust is increasingly popular for applications in finance, both due to its speed and memory safety, which make it more reliable than more traditional C++ code.

## The project

The aim of this project is to study pricing methods and estimates for some basic financial instruments.

### Notation

Through this document, we refer to various quantities using the following notation:

* $S$ is an asset in the market. $S_t$ is the price of $S$ at time $t$ (measured in years). We will assume $\ln(S_t/S_0)$ is normally distributed with mean $\mu t$ and variance $\sigma^2t$;

* $\rho$ is the annualised continuously compounded risk-free interest rate of the market;

* $\pi_{\tau}$ is the price of a European claim on the asset $S$ at the strike time $\tau$.

### Black-Scholes

The celebrated [Black-Scholes model](https://en.wikipedia.org/wiki/Black-Scholes_model) describes the dynamics of certain classes of options under assumptions about the market and underlying asset. In particular, it can be used to derive the price of a European-style option under idealised market conditions, leading to the famous Black-Scholes equation, which we have implemented in the ```black_scholes``` module as ```black_scholes_price```.

Letting $\pi$ be a European call with strike time $\tau$ and strike price $K$, we can price it as 

$$\pi_{\tau} = D_{\tau}\times(\Phi(d_+)F - \Phi(d_-)K)$$

where

$$d_+ = \frac{\ln(F/K) + \sigma^2\tau/2}{\sigma\sqrt{\tau}}$$

$$d_- = d_+ - \sigma\sqrt{\tau}$$

$$D_{\tau} = e^{-\rho\tau} \text{ is the discount factor}$$

$$F = S_0/{D_{\tau}} \text{ is the forward price of } S$$

and $\Phi$ is the cdf of a standard normal distribution.

### Bernoulli approximation

When pricing other options, like American-style options, there is no closed form expression. However, we can approximate the price accurately using a [binomial options pricing model](https://en.wikipedia.org/wiki/Binomial_options_pricing_model), also known as a Bernoulli approximation. The central idea is to replace the continuous Brownian motion with a discrete simple random walk. The interval $[0, \tau]$ is broken into steps $[0, \frac{\tau}{n}, \frac{2\tau}{n}, ..., \tau]$, and at each step the increment in the logarithm of the price is assumed to be $g$ (with probability $p$) or $-g$ (with probability $1-p$). $p$ and $g$ are chosen so that the increment has mean $\mu\tau/n$ and variance $\sigma^2\tau/n$ - these are derived in [Problem two](#problem-two).

## Problems

The original CATAM project involved certain explicit questions and problems, which are reproduced (and solved) here.

### Problem one: 

Write a routine to evaluate the Black-Scholes price. Compile a table of the price of European calls when the strike price is $\tau = 40$, strike time is $\tau = 2$ or $3$, the spot price of the underlying is $S_0 = 52, 100$ or $107$, the volatility of the underlying is $\sigma = 0.5$, and the interest rate is $\rho = 0.035$.

#### Solution: 

We generate our results using the following code:

```rust
fn main() -> Result<(), Box<&'static str>> {
    for spot_price in [52.0,100.0,107.0].iter() {
        let underlying = asset::Underlying::new(*spot_price, 0.5, 0.035);
        for expiry_time in [2.0, 3.0].iter() {
            let call = asset::EuropeanCall::new(40.0, *expiry_time, underlying);
            println!("spot: {}, strike time: {}, bsp: {:.3}", spot_price, expiry_time, call.black_scholes_price());
        }
    }
    Ok(())
}
```

The table of results is given below:

| $S_0$ | $\pi_2$ | $\pi_3$ |
| ----- | ------- | ------- |
| 52    | 20.909  | 23.858  |
| 100   | 64.209  | 66.781  |
| 107   | 70.959  | 73.440  |

---

### Problem two:

For a Bernoulli approximation, calculate $g$ and $p$ as functions of the parameters.

#### Solution:

Let $\hat{S}_k$ be the value of our random walk after $k$ steps. Writing $I_1 = \ln(\hat{S}_1/\hat{S}_0)$, we can compute

$$\mu\tau/n = \mathbb{E}(I_1) = gp + (-g)(1-p) = g(2p-1)$$

and

$$\sigma^2\tau/n = \mathrm{Var}(I_1) = 4p(1-p)g^2$$

So we would like

$$\lim_{n\rightarrow\infty} g(2p-1)n = \mu\tau$$

$$\lim_{n\rightarrow\infty} 4p(1-p)g^2n = \sigma^2\tau$$

It is straightforward to see this is solved by $g = \sigma\sqrt{\frac{\tau}{n}}$, $p = \frac{1}{2} + \frac{\mu}{2\sigma}\sqrt{\frac{\tau}{n}}$.

---

