use rs_stats::StatsError;
use rs_stats::distributions::normal_distribution::normal_cdf;
use std::f64::consts::E;

use asset::EuropeanCall;

/** Discount for an asset.

    Parameters
    ----------
    * `call` : A European call on an underlying asset.
    * `interest_rate` : The continuously compounded risk-free interest rate.

    Returns
    -------
    * `f64` : The stock moneyness of the underlying, under the appropriate risk-neutral probability measure.
*/
fn discount_factor(strike_time: f64, interest_rate: f64) -> f64 {
    let discount = E.powf(-strike_time*interest_rate);
    return discount
}

/** Stock moneyness for a European call.

    Parameters
    ----------
    * `call` : A European call on an underlying asset.
    * `interest_rate` : The continuously compounded risk-free interest rate.

    Returns
    -------
    * `f64` : The stock moneyness of the underlying, under the appropriate risk-neutral probability measure.
*/
fn stock_moneyness(call: &EuropeanCall, interest_rate: f64) -> f64 {
    let underlying = &call.underlying;
    let fwd_price = underlying.fwd_price(call.strike_time, interest_rate);
    let moneyness = 1.0/(underlying.std_dev*call.strike_time.sqrt()) * ((fwd_price/call.strike_price).ln() + (interest_rate + underlying.std_dev.powf(2.0)/2.0)*call.strike_time);
    return moneyness
}

/** RFA moneyness for a European call.

    Parameters
    ----------
    * `call` : A European call on an underlying asset.
    * `interest_rate` : The continuously compounded risk-free interest rate.

    Returns
    -------
    * `f64` : The risk-free moneyness of the underlying, under the appropriate risk-neutral probability measure.
*/
fn rfa_moneyness(call: &EuropeanCall, interest_rate: f64) -> f64 {
    let underlying = &call.underlying;
    let moneyness = stock_moneyness(call, interest_rate) - underlying.std_dev*call.strike_time.sqrt();
    return moneyness
}

/** Computes the Black-Scholes price for a European call.

    Parameters
    ----------
    * `call` : A European call on an underlying asset.
    * `interest_rate` : The continuously compounded risk-free interest rate.

    Returns
    -------
    * `f64` : The price of the call.
*/
pub fn black_scholes_price(call: &EuropeanCall, interest_rate: f64) -> Result<f64,StatsError> {
    let std_norm_cdf = |x: f64| normal_cdf(x, 0.0, 1.0);
    let underlying = &call.underlying;
    let fwd_price = underlying.fwd_price(call.strike_time, interest_rate);

    let discount = discount_factor(call.strike_time, interest_rate);
    let undiscounted_price = std_norm_cdf(stock_moneyness(call, interest_rate))?*fwd_price - std_norm_cdf(rfa_moneyness(call, interest_rate))?*call.strike_price;

    let price = discount * undiscounted_price;
    return Ok(price)
}
