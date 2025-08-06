// cargo run --example 02_strategy

// Choosing behavior at runtime
// Works great where you don’t know behavior at compile-time.
// Rust’s trait objects (Box<dyn Trait>) provide dynamic dispatch and make this elegant.

trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

struct CreditCard;
impl PaymentStrategy for CreditCard {
    fn pay(&self, amount: f64) {
        println!("Paid €{amount} using Credit Card");
    }
}
struct Paypal;
impl PaymentStrategy for Paypal {
    fn pay(&self, amount: f64) {
        println!("Paid €{amount} via PayPal");
    }
}
struct PaymentContext {
    strategy: Box<dyn PaymentStrategy>,
}
impl PaymentContext {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        Self { strategy }
    }
    fn process(&self, amount: f64) {
        self.strategy.pay(amount);
    }
}

// We create two PaymentContext, each with a different strategy (CreditCard or Paypal).
// Each context calls process(amount), which delegates to the corresponding strategy's pay() method.
fn main() {
    // Use Credit Card payment strategy
    let credit_card_payment = PaymentContext::new(Box::new(CreditCard));
    credit_card_payment.process(100.0);

    // Use PayPal payment strategy
    let paypal_payment = PaymentContext::new(Box::new(Paypal));
    paypal_payment.process(75.5);
}
