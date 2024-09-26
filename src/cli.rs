mod cloud_flight;

fn main() {
    let cf = cloud_flight::CloudFlight::new();

    cf.battery();
    cf.read();

    println!("{}", cf.battery.get());
}
