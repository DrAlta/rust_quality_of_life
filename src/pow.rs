trait Exponent<T> {
    
    
    fn power(self, value: T) -> T;
}

impl Exponent<f64> for i32 {
    fn power(self, value: f64) -> f64{
        value.powi(self)
    }
}
impl Exponent<f64> for f64 {
    fn power(self, value: f64) -> f64{
        value.powf(self)
    }
}

impl Exponent<f32> for i32 {
    fn power(self, value: f32) -> f32 {
        value.powi(self)
    }
}
impl Exponent<f32> for f32 {
    fn power(self, value: f32) -> f32 {
        value.powf(self)
    }
}


pub trait Pow<T> {
    fn pow(self, power:T) -> Self ;
}

impl<P:Exponent<T>, T> Pow<P> for T {
    fn pow(self, power:P) -> Self {
        power.power(self)
    }

}


