struct BankAccount{
    balance: f64,
}

impl BankAccount{
    fn deposit(&mut self, amount: f64){
        self.balance = self.balance + amount;
    }
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        self.balance = self.balance - amount;
    }
}

fn main(){
    let mut account = BankAccount{
        balance: 100.0,
    };
    
    account.deposit(150.0);
    account.withdraw(33.0);
}


