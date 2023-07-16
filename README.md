# Brief Review

# Intro:-
Blockchain Technology is a method of recording information under a decentralised system which is impossible to change, manipulate or hacked. Blockchain is a  distributed ledger that duplicates and distribute the transactional information to all the connected networks. It is a structure that stores all records of transaction in a single block which is connected to the next block of transaction in a chain manner which is termed as Blockchain. This storage is known as Digital Ledger. It is highly secured, distributed, immutable, decentralised and transparent.

Every transaction in a ledger is authorised by the digital signature of the owner which authenticates the transaction and safeguards from tampering. Hence, its highly secured.

# Structure of Blockchain:-

A Block consists of three major components:
1) Header - contains metadata such as timpestamp and previous block's hash.
2) Data section - contains the actual information such as transactions and smart contracts.
3) Hash - its a unique cryptographic value of the entire block for verification purposes.

# Process of Transaction:-

When two people want to perform a transaction using their private and public keys, respectively, the first party would attach the transaction data to the second party's public key. The block includes a timestamp, a digital signature, and other crucial details. It should be emphasised that the block does not contain information on the parties to the transaction's identities. The block is then sent around the entire network, and when the correct user uses his private key to match it with the block, the transaction is successfully completed.

The Blockchain is able to record financial transactions as well as prperties, cars, and so on which is known as smart contracts.





# Introduction
In this assignment, you will be using your knowledge of the C++ programming language to implement a simple blockchain. A blockchain is a digital ledger of transactions that is decentralized and secure. Each block in the chain contains a cryptographic hash of the previous block, along with the current transactions.

# Requirements
Implement a basic blockchain data structure in C++. The blockchain should consist of blocks, each with a unique hash and a pointer to the previous block in the chain.
Each block should contain a list of transactions, represented as a linked list.
Implement a function to add a new block to the chain. This function should take a list of transactions as input, calculate the hash of the previous block, and add the new block to the chain.
Implement a function to validate the blockchain. This function should traverse the chain and verify that each block's hash matches the previous block's hash.
Test your blockchain implementation by adding some transactions and validating the chain.
Research and write a brief report on the security implications of using a blockchain for financial transactions.
Submission Guidelines
Fork this repository and create a new branch for your work.
Create a folder with your name and place all your files inside it.
Write all your code in C++ language.
Include a README.md file explaining how to compile and run your code.
Write a brief report on the security implications of using a blockchain for financial transactions and include it in your folder.
Submit a pull request to the master branch of the original repository.

# Evaluation
Your assignment will be evaluated based on the following criteria:

Correctness and completeness of the blockchain implementation
Clarity and readability of the code
Correctness and completeness of the validation function
Quality and completeness of the report on blockchain security

