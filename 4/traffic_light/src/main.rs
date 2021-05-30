// enum definition
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// trait Duration declaration
trait Duration {
    fn duration(&self) -> i32;
}

// implement the trait for the enum
impl Duration for TrafficLight {
    fn duration(&self) -> i32 {
        match self {
            TrafficLight::Red    => 5,
            TrafficLight::Yellow => 6,
            TrafficLight::Green  => 7,
        }
    }
}

pub fn print_duration(traffic_light: TrafficLight) {
    println!("The duration is {}", traffic_light.duration());
}

fn main() {
    print_duration(TrafficLight::Red);
    print_duration(TrafficLight::Yellow);
    print_duration(TrafficLight::Green);
}
