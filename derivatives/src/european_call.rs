use crate::sim_funs;

use statrs::distribution::{Normal, Continuous, ContinuousCDF};

pub struct BlackScholes {
    sigma : f64,
    r: f64,
    s0: f64,
    k_strike: f64,
    t_mat: f64,
}

impl BlackScholes {
    pub fn new(sigma: f64, r: f64, s0: f64, k_strike: f64, t_mat: f64) -> Self {
        if sigma <= 0.0 {
            panic!("sigma can not be negative");
        }
        if t_mat <= 0.0 {
            panic!("t_mat can not be negative");
        }
        Self {sigma:sigma, r:r, s0:s0 , k_strike:k_strike, t_mat:t_mat}
    }

    fn payoff(s: f64, k: f64) -> f64 {
        if s > k {
            return s-k;
        }
        return 0.0;
    }

    pub fn exact_price(&self) -> f64 {
        let mut norm = Normal::standard();
        let d_plus = ( 1.0/(self.sigma * self.t_mat.sqrt()) )*( (self.s0/self.k_strike).ln() + (self.r + self.sigma.powi(2)/2.0)*self.t_mat );
        let d_minus = d_plus - self.sigma*self.t_mat.sqrt();
        let price = norm.cdf(d_plus)*self.s0 - norm.cdf(d_minus)*self.k_strike*(-self.r*self.t_mat).exp();
        return price;
    }

    pub fn mc_price(&self, num_path: u32) -> f64 {
        let time_points: Vec<f64> = vec![self.t_mat];
        let mut acc: f64 = 0.0;
        for i in 0..num_path {
            let st: f64 = sim_funs::sim_gbm(self.r, self.sigma, self.s0, &time_points)[0];
            let poff: f64 = BlackScholes::payoff(st, self.k_strike);
            acc += poff;
        }
        let mean_acc: f64 = acc / num_path as f64;
        let price: f64 = mean_acc * (-self.r*self.t_mat).exp();
        return price;
    }

    pub fn mc_price_fast(&self, num_sim: usize) -> f64 {
        let gbm_vec: Vec<f64> = sim_funs::sim_gbm_at_t(self.r, self.sigma, self.s0, self.t_mat, num_sim);
        let payoff_sum:f64 = gbm_vec.iter().map(|s| BlackScholes::payoff(*s, self.k_strike) ).sum();
        let mean_payoff: f64 = payoff_sum/num_sim as f64;
        let price: f64 = mean_payoff * (-self.r*self.t_mat).exp();
        return price;
    } 
} 