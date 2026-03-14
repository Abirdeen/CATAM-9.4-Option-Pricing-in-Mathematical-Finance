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

    Returns
    -------
    * `f64` : The stock moneyness of the underlying, under the appropriate risk-neutral probability measure.
*/
fn stock_moneyness(call: &EuropeanCall) -> f64 {
    let underlying = &call.underlying;
    let fwd_price = underlying.fwd_price(call.expiry_time);
    let moneyness = ((fwd_price/call.strike_price).ln() + (underlying.interest_rate + underlying.std_dev.powf(2.0)/2.0)*call.expiry_time) / (underlying.std_dev*call.expiry_time.sqrt());
    return moneyness
}

/** RFA moneyness for a European call.

    Parameters
    ----------
    * `call` : A European call on an underlying asset.

    Returns
    -------
    * `f64` : The risk-free moneyness of the underlying, under the appropriate risk-neutral probability measure.
*/
fn rfa_moneyness(call: &EuropeanCall) -> f64 {
    let underlying = &call.underlying;
    let moneyness = stock_moneyness(call) - underlying.std_dev*call.expiry_time.sqrt();
    return moneyness
}

/** Computes the Black-Scholes price for a European call.

    Parameters
    ----------
    * `call` : A European call on an underlying asset.

    Returns
    -------
    * `f64` : The price of the call.
*/
pub fn black_scholes_price(call: &EuropeanCall) -> Result<f64,StatsError> {
    let std_norm_cdf = |x: f64| normal_cdf(x, 0.0, 1.0);
    let underlying = &call.underlying;
    let fwd_price = underlying.fwd_price(call.expiry_time);

    let discount = underlying.discount_factor(call.expiry_time);
    let undiscounted_price = std_norm_cdf(stock_moneyness(call))?*fwd_price - std_norm_cdf(rfa_moneyness(call))?*call.strike_price;

    let price = discount * undiscounted_price;
    return Ok(price)
}
