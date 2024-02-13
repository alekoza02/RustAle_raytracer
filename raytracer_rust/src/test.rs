use std::f64::consts::PI;

fn main() {
    
    let v1 : [f64; 3] = [0.7, -0.7, 0.0];
    let v2 : [f64; 3] = [0.0, 1.0, 0.0];

    let cos = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
    let theta = - cos.asin();
    let theta_ang = theta * 180.0 / PI;

    println!("cos -> {}, theta -> {}", cos, theta_ang);
}
