# Polkadot-Ledger
Creating a ledger on Polkadot involves writing and deploying a custom smart contract using the Substrate framework. Here's an example of how you can create a simple ledger smart contract in Rust using the FRAME framework:

This code defines a simple ledger smart contract with a storage item called LedgerData that maps a key (represented by a Vec<u8>) to a value (also represented by a Vec<u8>). The store_data function allows an authorized user to store data in the ledger by providing a key-value pair.

To deploy this smart contract on a Polkadot network, you would need to compile it using the Rust compiler and deploy it using the appropriate deployment tools for the specific network you are targeting.

  In a real-world scenario, you would need to consider additional security, access control, and governance mechanisms to ensure the integrity and functionality of the ledger.
