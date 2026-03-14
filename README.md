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

