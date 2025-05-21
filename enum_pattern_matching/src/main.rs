#[derive(Debug)]
enum TrafficLight 
{
    Red,
    YellowGoingToGreen,
    Green,
    YellowGoingToRed,
}

impl TrafficLight 
{
    fn next(self) -> TrafficLight 
    {
        match self 
        {
            TrafficLight::Red                 => TrafficLight::YellowGoingToGreen,
            TrafficLight::YellowGoingToGreen  => TrafficLight::Green,
            TrafficLight::Green               => TrafficLight::YellowGoingToRed,
            TrafficLight::YellowGoingToRed    => TrafficLight::Red,
        }
    }

    fn name(&self) -> &'static str 
    {
        match self 
        {
            TrafficLight::Red                 => "Red",
            TrafficLight::Green               => "Green",
            TrafficLight::YellowGoingToGreen
            | 
            TrafficLight::YellowGoingToRed    => "Yellow",
        }
    }
}

fn main() 
{
    let mut light = TrafficLight::Red;
    println!("Starting at {}", light.name());

    for _ in 0..7 
    {
        light = light.next();
        println!("now     {}", light.name());
    }
}

