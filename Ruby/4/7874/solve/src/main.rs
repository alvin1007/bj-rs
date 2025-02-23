use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Sphere {
    x: f64,
    y: f64,
    z: f64,
    r: f64,
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        let mut spheres = [Sphere { x: 0.0, y: 0.0, z: 0.0, r: 0.0 }; 3];
        
        for i in 0..3 {
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            let data = buf.split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<_>>();
            spheres[i] = Sphere { x: data[0], y: data[1], z: data[2], r: data[3] };
        }
        
        println!("{:.6}", volume);
    }
}
