#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub user: Address,
    pub course_id: u64,
    pub subscribed_at: u64,
    pub duration_days: u64,
}

#[contracttype]
pub enum SubscriptionKey {
    Sub(Address, u64), // (user, course_id)
    SubCount,
}

#[contract]
pub struct SubscriptionLearning;

#[contractimpl]
impl SubscriptionLearning {
    pub fn subscribe(env: Env, user: Address, course_id: u64, duration_days: u64) {
        user.require_auth();

        if duration_days == 0 {
            panic!("Duration must be greater than 0");
        }

        let key = SubscriptionKey::Sub(user.clone(), course_id);
        let subscription = Subscription {
            user,
            course_id,
            subscribed_at: env.ledger().timestamp(),
            duration_days,
        };

        env.storage().instance().set(&key, &subscription);

        let mut count: u64 = env.storage().instance().get(&SubscriptionKey::SubCount).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&SubscriptionKey::SubCount, &count);
    }

    pub fn check_subscription(env: Env, user: Address, course_id: u64) -> bool {
        let key = SubscriptionKey::Sub(user, course_id);
        match env.storage().instance().get::<_, Subscription>(&key) {
            Some(sub) => {
                let now = env.ledger().timestamp();
                let valid_until = sub.subscribed_at + (sub.duration_days * 86400);
                now <= valid_until
            },
            None => false,
        }
    }

    pub fn total_subscriptions(env: Env) -> u64 {
        env.storage().instance().get(&SubscriptionKey::SubCount).unwrap_or(0)
    }
}
