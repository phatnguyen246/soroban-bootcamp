#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, String, symbol_short};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    TotalSupply,
    Name,
    Symbol,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAdmin = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
}

#[contract]
pub struct MyToken;

#[contractimpl]
impl MyToken {
    /// Gọi một lần khi contract được deploy
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
    }

    /// Admin tạo token mới và gửi đến một địa chỉ
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        let balance: i128 = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(to.clone()), 29 * DAY, 30 * DAY);
        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
        env.events().publish((symbol_short!("mint"), to), amount);
        Ok(())
    }

    /// Chuyển token từ tài khoản này sang tài khoản khác
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        from.require_auth(); // Người gửi phải ký giao dịch này
        let from_bal = Self::balance(env.clone(), from.clone());
        if from_bal < amount { return Err(Error::InsufficientBalance); }
        let to_bal = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_bal - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_bal + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(from.clone()), 29 * DAY, 30 * DAY);
        env.storage().persistent().extend_ttl(&DataKey::Balance(to.clone()), 29 * DAY, 30 * DAY);
        env.events().publish((symbol_short!("transfer"), from, to), amount);
        Ok(())
    }

    /// Kiểm tra số dư của bất kỳ địa chỉ nào (ai cũng có thể gọi)
    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(addr)).unwrap_or(0)
    }

    pub fn name(env: Env) -> String { env.storage().instance().get(&DataKey::Name).unwrap() }
    pub fn symbol(env: Env) -> String { env.storage().instance().get(&DataKey::Symbol).unwrap() }
    pub fn total_supply(env: Env) -> i128 { env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0) }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_full_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(MyToken, ());
        let client = MyTokenClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        // Khởi tạo
        client.initialize(
            &admin,
            &String::from_str(&env, "Campus Coin"),
            &String::from_str(&env, "CAMP"),
        );

        // Mint 1000 cho Alice
        client.mint(&alice, &1000);
        assert_eq!(client.balance(&alice), 1000);

        // Alice gửi 300 cho Bob
        client.transfer(&alice, &bob, &300);
        assert_eq!(client.balance(&alice), 700);
        assert_eq!(client.balance(&bob), 300);
    }
}
