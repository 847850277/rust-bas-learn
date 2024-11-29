//Invoker
struct BankClient {
    put_command: Box<dyn Command>,
    get_command: Box<dyn Command>,
}
impl BankClient {
    fn put_money(&self) {
        self.put_command.execute();
    }
    fn get_money(&self) {
        self.get_command.execute();
    }
}
//Receiver
#[derive(Copy, Clone)]
struct Bank {}
impl Bank {
    fn give_money(&self) {
        println!("money to the client");
    }
    fn receive_money(&self) {
        println!("money from the client");
    }
}
//Command
trait Command {
    fn execute(&self);
}
//ConcreteCommand
struct PutCommand {
    bank: Bank,
}
impl Command for PutCommand {
    fn execute(&self) {
        self.bank.give_money();
    }
}
//ConcreteCommand
struct GetCommand {
    bank: Bank,
}
impl Command for GetCommand {
    fn execute(&self) {
        self.bank.receive_money();
    }
}
pub(crate) fn test() {
    let bank = Bank {};
    let c_put = PutCommand { bank: bank };
    let c_get = GetCommand { bank: bank };
    let client = BankClient {
        put_command: Box::new(c_put),
        get_command: Box::new(c_get),
    };
    client.get_money();
    //printed: money to the client
    client.put_money();
    //printed: money from the client
}
