use core::f64;
use crate::asset::{EuropeanCall, CallOption};

pub trait BernoulliApproxable: CallOption {

    fn step_price(&self, i: usize, j: usize, steps: usize, 
        step_prices: &Vec<Vec<f64>>) -> f64;

    fn approximate_price(&self, steps: usize) -> f64 {
        let mut step_prices = vec![vec![f64::NAN;steps];steps];
        for i in 0..steps {
            for j in 0..i {
                step_prices[steps - i][j] = self.step_price(steps - i, j, steps, &step_prices)
            }

        }
        return step_prices[0][0]
    }
}

impl BernoulliApproxable for EuropeanCall {

    fn step_price(&self, i: usize, j: usize, steps: usize, 
        step_prices: &Vec<Vec<f64>>) -> f64 {
        let underlying = self.underlying;
        let step_time = self.expiry_time / steps as f64;
        if i == steps {
            let spot_price = underlying.spot_price;
            let strike_price = self.strike_price;
            let g=underlying.std_dev * step_time.sqrt();
            return (spot_price * ((2*j - steps) as f64 * g).exp() - strike_price).max(0.0)
        } else {
            let p = 0.5 + 0.5*(underlying.mean()/underlying.std_dev)*step_time.sqrt();
            let discount = underlying.discount_factor(step_time);
            return discount * (p*step_prices[i+1][j+1] + (1.0-p)*step_prices[i+1][j])
        }
    }
}
