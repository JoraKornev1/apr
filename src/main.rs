struct Token {
    amount: f64,
    apr: f64,
    redelegate_days: f64,
    period: f64,
}
impl Token {
    fn apr (&self) -> f64 {
        let mut day = 0.0;
        let mut _ammount = self.amount;
        while day < self.period {
           let day_ammount = _ammount * self.apr / 365.0;
           _ammount = _ammount + day_ammount * self.redelegate_days;
            day = day + self.redelegate_days
        }
        _ammount
    }
}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn apr_check() {
        let x = Token {
            amount: 1.0,
            apr: 1.0,
            redelegate_days: 1.0,
            period: 1.0
        };
        let result:f64 = 1.0027397260273974;

        assert_eq!(x.apr(), result);


    }

}
fn main() {
    let token = Token {
        amount: 32.0, // Use only float number
        apr: 1.1, // Use only float number
        redelegate_days: 1.0, // Use only float number
        period: 365.0 // Use only float number
    };
    println!("result: {}", token.apr());
}
