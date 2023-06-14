# Introduction
This repository contrain Assessment Submission of task given by HogoByte.

## Blockchain Ledger
This is a simple implementation of a blockchain ledger in C++. It allows you to add transactions, create blocks, validate the ledger, and print the ledger.

## Prerequisites
To compile and run this code, you need to have the following installed on your system:

- C++ compiler (supporting C++11 or later)
- OpenSSL library


## Compilation
To compile the code, follow these steps:

1. Clone the repository or download the source code files.
2. Open a terminal or command prompt.
3. Navigate to the directory where the source code files are located.
4. Run the following command to compile the code:
  - g++ blockchain.cpp -o program -lssl -lcrypto  (This command uses the g++ compiler, sets the C++ version to C++11, and links against the OpenSSL library.)
  
## Execution
1. Open a terminal or command prompt.
2. If there are no compilation errors, an executable file named `program` will be generated in the same directory.
   Navigate to the directory where the `program` executable is located.
   - ./program
 
## Usage
Here's a brief description of the available operations:

1. **Add Transaction**: Allows you to add a transaction to the transaction pool.
2. **Add Block**: Creates a new block with the transactions in the transaction pool and adds it to the blockchain.
3. **Validate Ledger**: Checks the integrity and validity of the blockchain.
4. **Print Ledger**: Displays the entire blockchain with block details and transactions.
5. **Exit**: Quits the program.

## Example
Starting Blockchain Ledger...

Enter a number to perform an operation:
1: Add Transaction
2: Add Block
3: Validate Ledger
4: Print Ledger
5: Exit

1

Enter Transaction: Abhishek to Rajat, 50 INR
Transaction Successfully added..

Enter a number to perform an operation:
1: Add Transaction
2: Add Block
3: Validate Ledger
4: Print Ledger
5: Exit

1

Enter Transaction: Abhishek to Swapnil, 40 INR
Transaction Successfully added..

Enter a number to perform an operation:
1: Add Transaction
2: Add Block
3: Validate Ledger
4: Print Ledger
5: Exit

2

Block successfully added to Blockchain...

Enter a number to perform an operation:
1: Add Transaction
2: Add Block
3: Validate Ledger
4: Print Ledger
5: Exit

4

Block Index: 1
Block Hash:  2f64366f6d4433eaf48293313f1f82668afcee78f5f0394042dda4327a20cc8e
Previous Hash: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
Transactions:
  - Abhishek to Swapnil, 40 INR
  - Abhishek to Rajat, 50 INR
------------------------
Block Index: 0
Block Hash:  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
Previous Hash: 
Transactions: 

-------------------------
Enter a number to perform an operation:
1: Add Transaction
2: Add Block
3: Validate Ledger
4: Print Ledger
5: Exit

3

Blockchain is valid.

Enter a number to perform an operation:
1: Add Transaction
2: Add Block
3: Validate Ledger
4: Print Ledger
5: Exit



