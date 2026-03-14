use rs_stats::StatsError;
use rs_stats::distributions::normal_distribution::normal_cdf;

use asset::EuropeanCall;
use asset::CallOption;

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

pub trait BS86: CallOption {
    fn black_scholes_price(&self) -> f64;
}

impl BS86 for EuropeanCall {
    fn black_scholes_price(&self) -> f64 {
        let price = black_scholes_price(self);
        match price {
            Ok(price) => return price,
            Err(error) => panic!("{}", error)
        }
    }
}