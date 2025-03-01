use rand::{thread_rng};
use rand::distributions::{Distribution, Normal};


pub fn sim_bm(time_points: &Vec<f64>) -> Vec<f64> {
    let N = time_points.len();
    let normal = Normal::new(0.0,1.0);
    let z: Vec<f64> = normal.sample_iter(&mut thread_rng()).take(N).collect();
    let mut t_diff: Vec<f64> = time_points.windows(2).map(|x| x[1] - x[0]).collect();
    t_diff.insert(0, time_points[0]);

    let it = z.iter().zip(t_diff.iter());
    let prod_it = it.map( |( zi , t_diffi )| zi*t_diffi.sqrt() );
    let mut acc: f64 = 0.0;
    let mut res: Vec<f64> = vec![]; 
    for x in prod_it {
        acc += x;
        res.push(acc.clone());
    }

    return res;
}


pub fn sim_gbm(mu: f64, sigma: f64, s0: f64, time_points: &Vec<f64>) -> Vec<f64> {
    let bm_vec: Vec<f64> = sim_bm(&time_points);
    let it = time_points.iter().zip(bm_vec.iter());
    let res_vec:Vec<f64> = it.map(|(t, w)| s0*((mu - sigma.powi(2)/2.0)*t + sigma*w).exp()).collect();
    return res_vec;
}

pub fn sim_bm_at_t(time_point: f64, num_sim: usize) -> Vec<f64>
{
    let normal = Normal::new(0.0,1.0);
    let z: Vec<f64> = normal.sample_iter(&mut thread_rng()).take(num_sim).collect();
    let res_vec: Vec<f64> = z.iter().map(|zi| zi*time_point.sqrt() ).collect();
    return res_vec;
}

pub fn sim_gbm_at_t(mu: f64, sigma: f64, s0: f64, time_point: f64, num_sim: usize) -> Vec<f64> {
    let bm_vec: Vec<f64> = sim_bm_at_t(time_point, num_sim);
    let res_vec: Vec<f64> = bm_vec.iter().map( |w| s0*((mu - sigma.powi(2)/2.0)*time_point + sigma*w).exp() ).collect();
    return res_vec;
}