# 🤖 Piker

Jumpstart your next liquidation bot with Piker

### Description

The modern Rust boilerplate for building reliable, asynchronous and slim
liquidation bots for Pike protocol

### Dependencies

- Latest Rust for speed and robustness
- Tokio for async operations

### Main workflow

1. **Initialization**: The bot begins by establishing a connection to the RPC
   provider via HTTP or WebSocket. This allows the bot to interact with the
   blockchain and the Pike protocol's smart contracts.

2. **Event Subscription**: Once connected, the bot subscribes to a stream of
   events. Add more events depending on your strategy!

3. **State Reconstruction**: As events are received, the bot parses each one and
   updates its internal representation accordingly. This allows the bot to
   maintain an up-to-date understanding of each user's position with minimal
   calls to the smart contracts, improving efficiency.

4. **Liquidation Check**: The bot checks for potential liquidations by examining
   the health factor of users' positions.

5. **Liquidation Execution**: If a user is found to be in a state where they can
   be liquidated (health factor is less than 1), the bot calls the Pike
   contract's liquidation function to initiate the liquidation process.

6. **Debt Liquidation**: The bot liquidates the user's debt, effectively
   repaying what they owe to the protocol.

7. **Continual Monitoring**: The cycle continues indefinitely.

### Community

Join Pike community to get help now!

- Discord

### Credits

@gakonst
