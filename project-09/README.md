# Subscription-based Learning

## Project Description

This smart contract provides the foundation for a decentralized subscription system for learning platforms. Users can subscribe to different learning modules or courses for a specified duration using blockchain-based validation.

## Project Vision

To empower decentralized, fair-access education by enabling blockchain-powered time-based subscriptions that are transparent, immutable, and censorship-resistant.

## Key Features

- **Time-Based Subscriptions**: Users subscribe to a course for a defined number of days.
- **Subscription Validity Check**: The system can verify if a user has an active subscription.
- **Immutable Ledger**: All subscription actions are stored transparently on-chain.
- **Basic Analytics**: Track the number of total subscriptions.

## Contract Details

Contract Address: CCG5FEHLSSRTCE3XYV7ZQ2SNY3DTACXTEOX4V73KCBAITQ2LW5OOR3YP

- `subscribe(user: Address, course_id: u64, duration_days: u64)`  
  Registers a user subscription to a specific course with a defined duration.

- `check_subscription(user: Address, course_id: u64) -> bool`  
  Checks if the user has an active subscription for the course.

- `total_subscriptions() -> u64`  
  Returns the total number of subscriptions registered on the platform.
