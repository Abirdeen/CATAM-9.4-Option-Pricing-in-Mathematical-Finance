#[derive(Clone, Copy)]
pub struct Underlying {
    pub spot_price: f64,
    pub std_dev: f64,
    pub interest_rate: f64
}

impl Underlying {
    pub fn new(spot_price: f64, std_dev: f64, interest_rate: f64) -> Underlying {
        return Underlying{spot_price, std_dev, interest_rate}
    }

    pub fn mean(&self) -> f64 {
        return self.interest_rate - (self.std_dev.powf(2.0)/2.0)
    }

    /** Discount for an asset.

        Parameters
        ----------
        * `time` : The time over which the discount is applied.

        Returns
        -------
        * `f64` : The discount.
    */
    pub fn discount_factor(&self, time: f64) -> f64 {
        return (-time*self.interest_rate).exp()
    }

    pub fn fwd_price(&self, time: f64) -> f64 {
        return self.spot_price/self.discount_factor(time)
    }
}

pub struct EuropeanCall {
    pub strike_price: f64,
    pub expiry_time: f64,
    pub underlying: Underlying
}

impl EuropeanCall {
    pub fn new(strike_price: f64, expiry_time: f64, underlying: Underlying) -> EuropeanCall {
        return EuropeanCall{strike_price, expiry_time, underlying}
    }
}

pub trait CallOption {
    fn underlying(&self) -> Underlying;
    fn expiry_time(&self) -> f64;
    fn strike_price(&self) -> f64;
}

impl CallOption for EuropeanCall {

    fn underlying(&self) -> Underlying {
        self.underlying
    }

    fn expiry_time(&self) -> f64 {
        self.expiry_time
    }

    fn strike_price(&self) -> f64 {
        self.strike_price
    }
}
