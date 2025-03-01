mod sim_funs;
mod european_call;

fn main() {
    //let time_points : Vec<f64> = vec![1.0,2.0,3.0];
    //let bm_vec: Vec<f64> = sim_funs::sim_bm( &time_points );
    //let gbm_vec:Vec<f64> = sim_funs::sim_gbm(0.0, 0.1, 100.0, &time_points);
    //println!("{:?}", gbm_vec);

    let sigma: f64 = 0.2;
    let r: f64 = 0.03;
    let s0: f64 = 100.0;
    let k_strike: f64 = 90.0;
    let t_mat: f64 = 1.0;
    let bs = european_call::BlackScholes::new(sigma, r, s0, k_strike, t_mat);
    let ec_exact_price: f64 = bs.exact_price();
    println!("Euro Call Exact Price : {}", ec_exact_price);

    //let ec_mc_price: f64 = bs.mc_price(10000);
    let ec_mc_price: f64 = bs.mc_price_fast(10000);
    println!("Euro Call MC Price : {}", ec_mc_price);



    //println!("Hello, world!");
}
