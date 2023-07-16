# Brief Review

# Intro:-
Blockchain Technology is a method of recording information under a decentralised system which is impossible to change, manipulate or hacked. Blockchain is a  distributed ledger that duplicates and distribute the transactional information to all the connected networks. It is a structure that stores all records of transaction in a single block which is connected to the next block of transaction in a chain manner which is termed as Blockchain. This storage is known as Digital Ledger. It is highly secured, distributed, immutable, decentralised and transparent.

Every transaction in a ledger is authorised by the digital signature of the owner which authenticates the transaction and safeguards from tampering. Hence, its highly secured.

Blockchain network connects all the peer-to-peer network via internet on a blockchain platform like Ethereum and Solana where we can create DApps. Each and every transactional details is visible on all other user's device except their profile. When the user commit a transaction, it attains finality and it can't be changed by anyone. Once if anyone tries to tamper the network or any transaction, it applies to their network alone and all other devices remains unchanged. 

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

To begin with the code implementation,
1) We have created a blockchain class and created an object for that.
2) Create a vector to store transactions in a block using push_back() also we can add new blocks.
3) Struct Transaction stores information of the sender, receiver, amount, timestamp.




