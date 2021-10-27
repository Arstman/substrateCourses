//the traffic light enum

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

//the Time trait, contains only one time method which returns the time of item will last
trait Time {
    fn time(&self) -> u8;
}
//impl Time for traffic lights
impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 55,
            TrafficLight::Yellow => 5,
        }
    }
}

fn main() {
    let r = TrafficLight::Red;
    let g = TrafficLight::Green;
    let y = TrafficLight::Yellow;

    //print times for each light
    println!("The Red Traffic Light lasts {} seconds. ", r.time());
    println!("The Green Traffic Light lasts {} seconds. ", g.time());
    println!("The Yellow Traffic Light lasts {} seconds. ", y.time());
}
