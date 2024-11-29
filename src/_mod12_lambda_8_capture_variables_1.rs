pub(crate) fn test() {
    let make_wallet = |mut sum| {
        move |pay| {
            sum -= pay;
            return sum;
        }
    };
    let sum1 = 1000;
    let mut pay_from_wallet1 = make_wallet(sum1);
    let sum2 = 500;
    let mut pay_from_wallet2 = make_wallet(sum2);
    let mut balance = pay_from_wallet1(50);
    //balance is 950
    println!("balance is {balance}");
    balance = pay_from_wallet2(70);
    //balance is 430
    println!("balance is {balance}");
    balance = pay_from_wallet1(150);
    //balance is 800
    println!("balance is {balance}");
}
