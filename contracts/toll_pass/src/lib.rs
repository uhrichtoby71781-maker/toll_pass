#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec,
};

/// Persistent storage keys for the TollPass contract.
#[contracttype]
pub enum DataKey {
    /// Prepaid balance of a driver's pass (credits).
    Balance(Address),
    /// Registration record of a toll station, keyed by station_id.
    Station(Symbol),
    /// A recorded crossing event, keyed by an incremental id.
    Crossing(u32),
    /// Index of crossing ids belonging to a driver.
    UserCrossings(Address),
    /// Auto-incrementing counter for crossing ids.
    NextCrossingId,
}

/// On-chain record of a toll station registered by an operator.
#[contracttype]
#[derive(Clone)]
pub struct Station {
    pub operator: Address,
    pub location: String,
    pub crossings: u32,
    pub revenue: u32,
}

/// On-chain record of a single highway crossing / fee deduction.
#[contracttype]
#[derive(Clone)]
pub struct Crossing {
    pub id: u32,
    pub user: Address,
    pub station_id: Symbol,
    pub fee: u32,
    pub timestamp: u64,
    pub disputed: bool,
    pub reason: String,
}

#[contract]
pub struct TollPass;

#[contractimpl]
impl TollPass {
    /// Top up a driver's prepaid toll pass with `amount` credits.
    /// Caller must be the pass holder (require_auth). Returns the new balance.
    pub fn topup(env: Env, user: Address, amount: u32) -> u32 {
        user.require_auth();
        if amount == 0 {
            panic!("amount must be > 0");
        }
        let key = DataKey::Balance(user.clone());
        let current: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        let new_balance = current.checked_add(amount).expect("balance overflow");
        env.storage().persistent().set(&key, &new_balance);
        new_balance
    }

    /// Register a new toll station owned by `operator` with a unique
    /// `station_id` and a human-readable `location`. Operator must sign.
    pub fn register_station(
        env: Env,
        operator: Address,
        station_id: Symbol,
        location: String,
    ) {
        operator.require_auth();
        let key = DataKey::Station(station_id.clone());
        if env.storage().persistent().has(&key) {
            panic!("station_id already registered");
        }
        let station = Station {
            operator,
            location,
            crossings: 0,
            revenue: 0,
        };
        env.storage().persistent().set(&key, &station);
    }

    /// Record a crossing at `station_id`, deducting `fee` credits from the
    /// driver `user`'s balance. The calling `station` address must match
    /// the station's registered operator. Returns the new crossing id.
    pub fn cross(
        env: Env,
        station: Address,
        user: Address,
        station_id: Symbol,
        fee: u32,
    ) -> u32 {
        station.require_auth();

        let station_key = DataKey::Station(station_id.clone());
        let mut s: Station = env
            .storage()
            .persistent()
            .get(&station_key)
            .expect("station not registered");
        if s.operator != station {
            panic!("caller is not the station operator");
        }

        let bal_key = DataKey::Balance(user.clone());
        let balance: u32 = env.storage().persistent().get(&bal_key).unwrap_or(0);
        if balance < fee {
            panic!("insufficient pass balance");
        }
        env.storage().persistent().set(&bal_key, &(balance - fee));

        s.crossings = s.crossings.checked_add(1).expect("crossings overflow");
        s.revenue = s.revenue.checked_add(fee).expect("revenue overflow");
        env.storage().persistent().set(&station_key, &s);

        let cid_key = DataKey::NextCrossingId;
        let prev: u32 = env.storage().persistent().get(&cid_key).unwrap_or(0);
        let new_id = prev.checked_add(1).expect("id overflow");

        let crossing = Crossing {
            id: new_id,
            user: user.clone(),
            station_id: station_id.clone(),
            fee,
            timestamp: env.ledger().timestamp(),
            disputed: false,
            reason: String::from_str(&env, ""),
        };
        env.storage()
            .persistent()
            .set(&DataKey::Crossing(new_id), &crossing);
        env.storage().persistent().set(&cid_key, &new_id);

        let uc_key = DataKey::UserCrossings(user);
        let mut list: Vec<u32> = env
            .storage()
            .persistent()
            .get(&uc_key)
            .unwrap_or(Vec::new(&env));
        list.push_back(new_id);
        env.storage().persistent().set(&uc_key, &list);

        new_id
    }

    /// Flag a previously recorded crossing as disputed with a written
    /// `reason`. Only the crossing's driver may dispute, and only once.
    pub fn dispute(env: Env, user: Address, crossing_id: u32, reason: String) {
        user.require_auth();
        let key = DataKey::Crossing(crossing_id);
        let mut c: Crossing = env
            .storage()
            .persistent()
            .get(&key)
            .expect("crossing not found");
        if c.user != user {
            panic!("not your crossing");
        }
        if c.disputed {
            panic!("crossing already disputed");
        }
        c.disputed = true;
        c.reason = reason;
        env.storage().persistent().set(&key, &c);
    }

    /// Read the remaining prepaid credits on a driver's toll pass.
    pub fn get_balance(env: Env, user: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    /// Return the total number of crossings recorded for a driver.
    pub fn get_crossing_count(env: Env, user: Address) -> u32 {
        let list: Vec<u32> = env
            .storage()
            .persistent()
            .get(&DataKey::UserCrossings(user))
            .unwrap_or(Vec::new(&env));
        list.len()
    }

    /// Fetch the on-chain record of a registered toll station.
    pub fn get_station(env: Env, station_id: Symbol) -> Station {
        env.storage()
            .persistent()
            .get(&DataKey::Station(station_id))
            .expect("station not registered")
    }

    /// Fetch the on-chain record of a single crossing by id.
    pub fn get_crossing(env: Env, crossing_id: u32) -> Crossing {
        env.storage()
            .persistent()
            .get(&DataKey::Crossing(crossing_id))
            .expect("crossing not found")
    }
}
