/*
Allow us to `println!("{:?}", …)`, `.clone()`, and `==`
#[derive(Debug)] lets you println!("{:?}", …).
#[derive(Clone)] gives you a .clone() implementation.
#[derive(PartialEq)] allows == comparisons.i
*/

#[derive(Debug, Clone, PartialEq)]
struct Address
{
    street: String,
    city: String,
    lat: f64,
    long: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Person
{
    f_name: String,
    l_name: String,
    age: u8,
    address: Address,
}

impl Person
{
    fn new(
        f_name: impl Into<String>,
        l_name: impl Into<String>,
        age: u8,
        address: Address,
    ) -> Person
    {
        Person
        {
            f_name: f_name.into(),
            l_name: l_name.into(),
            age,
            address,
        }
    }

    fn print_person(&self) -> String
    {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.f_name,
            self.l_name,
            self.age,
            self.address.street,
            self.address.city,
            self.address.lat,
            self.address.long,
        )
    }
}

fn main() 
{
    let location = Address
    {
        street: "Space moonstreet 123".into(),
        city: "Moon".into(),
        lat: 10.123123,
        long: 12.33333,
    };

    // print the constructor
    let s = Person::new("Simon","la in", 28, location);
    println!("{:#?}", s);

    // print using the method
    println!("{}", s.print_person());
}
