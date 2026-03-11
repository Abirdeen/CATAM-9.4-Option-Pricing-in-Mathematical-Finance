pub struct Underlying {
    pub fwd_price: f64,
    pub std_dev: f64
}

pub struct EuropeanCall {
    pub strike_price: f64,
    pub strike_time: f64,
    pub underlying: Underlying
}
