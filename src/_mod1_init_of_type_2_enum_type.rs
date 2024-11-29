pub fn test() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Planet {
        Mercury = 1,
        Venus,
        Earth,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum PreciousMetal {
        Platinum = 1,
        Gold,
        Silver,
    }

    let gold = PreciousMetal::Gold;
    let earth = Planet::Earth;

    let gold_value = gold as u32;

    println!("gold is {:#?}", gold_value);
    dbg!(gold_value);

    let earth_value = earth as u32;

    println!("earth is {:#?}", earth_value);
    dbg!(earth_value);
}
