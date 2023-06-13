# Blockchain Implementation

This is a simple implementation of a blockchain in C++. It consists of three main classes: `Transaction`, `Block`, and `Blockchain`. The code creates a blockchain and adds transactions to it, then prints the blockchain and validates its integrity.

## Classes

### Transaction

The `Transaction` class represents a transaction in the blockchain. It has three member variables: `from` (sender's address), `to` (recipient's address), and `amount` (transaction amount).

### Block

The `Block` class represents a block in the blockchain. It has three member variables: `hash` (block hash), `previousHash` (hash of the previous block), and `transactions` (vector of transactions in the block).

The `Block` class also has a `calculateHash` method, which calculates the hash of the block based on its data and the current timestamp.

### Blockchain

The `Blockchain` class represents the entire blockchain. It has one member variable: `chain`, which is a vector of `Block` objects.

The `Blockchain` class has methods to add a new block to the chain, validate the integrity of the chain, and print the chain.

## Main Function

The `main` function creates an instance of the `Blockchain` class, adds transactions to it, and then prints the blockchain. Finally, it validates the integrity of the chain and outputs the result.

## Usage

To compile and run the code, you can use a C++ compiler. For example, if you have GCC installed, you can run the following commands:

```
g++ -std=c++11 blockchain.cpp -o blockchain
./blockchain
```

## Output

The code will output the following:

```
Block Hash: 7196326434351090248
Previous Hash: 0
Transactions:

Block Hash: 6705378055088153941
Previous Hash: 7196326434351090248
Transactions:
-> From: Rupen
-> To: Rushi
-> Amount: 10

Block Hash: 16753461608619223854
Previous Hash: 6705378055088153941
Transactions:
-> From: Rushi
-> To: Paavan
-> Amount: 5

Block Hash: 13550149934763958909
Previous Hash: 16753461608619223854
Transactions:
-> From: Paavan
-> To: Rupen
-> Amount: 7

Chain is valid? Yes
```

This shows the details of each block in the blockchain, including the block hash, previous hash, and the transactions in the block. Finally, it indicates whether the chain is valid or not.

Please note that this is a simplified implementation for educational purposes and may not include all the features and security measures of a real-world blockchain implementation.
