struct Token {
    amount: f64,
    apr: f64,
    redelegate_days: f64,
    period: f64,
}
impl Token {
    fn apr (&self) -> f64 {
        let mut day = self.redelegate_days;
        let mut _ammount = self.amount;
        while day < self.period {
           let day_ammount = _ammount * self.apr / 365.0;
           _ammount = _ammount + day_ammount * self.redelegate_days;
            day = day + self.redelegate_days
        }
        _ammount
    }
}
fn main() {
    let token = Token {
        amount: 160.0, // Use only float number
        apr: 4.3, // Use only float number
        redelegate_days: 28.0, // Use only float number
        period: 365.0 // Use only float number
    };
    println!("result : {}", token.apr());
}
