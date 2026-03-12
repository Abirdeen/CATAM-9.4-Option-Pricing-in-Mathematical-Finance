pub struct Underlying {
    pub spot_price: f64,
    pub std_dev: f64
}

impl Underlying {
    pub fn new(spot_price: f64, std_dev: f64) -> Underlying {
        return Underlying{spot_price, std_dev}
    }

    pub fn fwd_price(&self, time: f64, interest: f64) -> f64 {
        return self.spot_price*(interest*time).exp()
    }
}

pub struct EuropeanCall {
    pub strike_price: f64,
    pub strike_time: f64,
    pub underlying: Underlying
}

impl EuropeanCall {
    pub fn new(strike_price: f64, strike_time: f64, underlying: Underlying) -> EuropeanCall {
        return EuropeanCall{strike_price, strike_time, underlying}
    }
}
