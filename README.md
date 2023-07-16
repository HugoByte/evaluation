# Brief Review

# Intro:-
Blockchain Technology is a method of recording information under a decentralised system which is impossible to change, manipulate or hacked. Blockchain is a  distributed ledger that duplicates and distribute the transactional information to all the connected networks. It is a structure that stores all records of transaction in a single block which is connected to the next block of transaction in a chain manner which is termed as Blockchain. This storage is known as Digital Ledger. It is highly secured, distributed, immutable, decentralised and transparent.

Every transaction in a ledger is authorised by the digital signature of the owner which authenticates the transaction and safeguards from tampering. Hence, its highly secured.

Blockchain network connects all the peer-to-peer network via internet on a blockchain platform like Ethereum and Solana where we can create DApps. Each and every transactional details is visible on all other user's device except their profile. When the user commit a transaction, it attains finality and it can't be changed by anyone. Once if anyone tries to tamper the network or any transaction, it applies to their network alone and all other devices remains unchanged. This feature ensures the integrity of the data and prevents double spending, providing a high level of security and trust in Blockchain types and sustainability.

# Structure of Blockchain:-

A Block consists of three major components:
1) Header - contains metadata such as timpestamp and previous block's hash.
2) Data section - contains the actual information such as transactions and smart contracts.
3) Hash - its a unique cryptographic value of the entire block for verification purposes.

The decentralised structure of a blockchain is a way to record transactions. Every block has a specific number of transactions within it, making it a node. Every transaction has a distinct hash value along with a digital signature, and each block is linked to the next block by the hash values of the previous block. Blocks are treated as linked lists in this instance.

# Process of Transaction:-

When two people want to perform a transaction using their private and public keys, respectively, the first party would attach the transaction data to the second party's public key. The block includes a timestamp, a digital signature, and other crucial details. It should be emphasised that the block does not contain information on the parties to the transaction's identities. The block is then sent around the entire network, and when the correct user uses his private key to match it with the block, the transaction is successfully completed.

The Blockchain is able to record financial transactions as well as prperties, cars, and so on which is known as smart contracts.

# Implementation of a simple Blockchain:-

A simple Blockchain class is defined.

- index: An integer to keep track of the block's position in the chain.
- data: A string representing the data to be stored in the block.
- timestamp: A string representing the time when the block is created.
- previous_hash: A string representing the hash of the previous block in the chain.
- hash: A string representing the hash of the current block. The hash is calculated based on the block's data and other properties.

1. Two essential structures are defined: Transaction and Block.

  Transaction: Represents a single transaction in the blockchain, with attributes for the sender, receiver, amount, and timestamp when the transaction occurred.
  Block: Represents a block in the blockchain, containing an index, timestamp, a vector of Transaction objects, a previous block's hash, and its own hash.

2. Blockchain contains a private member variable chain, which is a vector of Block objects. The constructor initializes the chain with the genesis block (block with index 0 and empty transactions).

4. The createGenesisBlock() method creates and returns the genesis block.

5. The addBlock() method is used to add a new block to the blockchain. It takes a vector of transactions as input and creates a new block with the provided transactions, linking it to the last block in the chain.

6. The validateChain() method iterates through the entire blockchain and checks if each block's hash matches the calculated hash and if the previous block's hash matches the stored previous hash. If any inconsistency is found, the method returns false, indicating that the blockchain is invalid. Otherwise, it returns true, indicating a valid blockchain.

7. In the main() function, an instance of the Blockchain class named blockchain is created.

8. Two blocks with different transactions are added to the blockchain using the addBlock() method.

9. The validateChain() method is called to check the validity of the blockchain.

10. The result of the validation is printed on the console.




