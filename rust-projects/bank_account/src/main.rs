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
        if amount > self.balance{
            println!("Error: withdrawal amount exceeds available balance.");
            return;
        }
        self.balance = self.balance - amount;
        }   
    }

fn main(){
    let mut account = BankAccount{
        balance: 0.0,
    };

    account.deposit(15.0);
    account.withdraw(13.0);
    println!("current balance in account: {}", account.balance);
}


