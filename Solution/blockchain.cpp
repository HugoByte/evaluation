#include "blockchain.h"
#include <iostream>
#include <functional>
#include <openssl/sha.h>
#include <openssl/evp.h>
#include <iomanip>

using namespace std;

Blockchain::Blockchain() {

    //Initialize the transactionPool to nullptr 
    transactionPool = nullptr;

    // Initialize the chain with a genesis block
    chainHead = new Block;
    chainHead->index = 0;
    chainHead->blockHash = calculateHash("");
    chainHead->previousHash = "";
    chainHead->transactions = nullptr;
}

Blockchain::~Blockchain() {
    // Delete the blockchain chain
    Block* currentBlock = chainHead;
    while (currentBlock != nullptr) {
        TransactionNode* currentTransaction = currentBlock->transactions;
        while (currentTransaction != nullptr) {
            TransactionNode* nextTransaction = currentTransaction->next;
            delete currentTransaction;
            currentTransaction = nextTransaction;
        }
        Block* nextBlock = currentBlock->previousBlock;
        delete currentBlock;
        currentBlock = nextBlock;
    }
}

void Blockchain::addBlock(TransactionNode* transactions) {
    // Create a new block
    Block* newBlock = new Block;

    // Concanate all transactions to stringBuilder and calculate Hash
    TransactionNode* tempHead = transactions;
    string transactionBuilder = "";
    while(tempHead != NULL) {
        transactionBuilder += tempHead->transaction;
        tempHead = tempHead->next;
    }
    string blockHash = calculateHash(transactionBuilder);

    // new block have it's own hash and previous block hash
    newBlock->blockHash = blockHash;
    newBlock->previousHash = chainHead->blockHash;
    newBlock->transactions = transactions;
    newBlock->index = chainHead->index + 1;

    // Update the chain head
    newBlock->previousBlock = chainHead;
    chainHead = newBlock;

    // Remove old transactions
    transactionPool = nullptr;
    cout<<"Block successfully added to Blockchain..."<<endl;
    cout<<endl;
}


void Blockchain::addTransaction(const string& transaction) {
    // Create a new transaction node
    TransactionNode* newTransaction = new TransactionNode(transaction);

    // Add the transaction to the current block's linked list of transactions
    newTransaction->next = transactionPool;
    transactionPool = newTransaction;
    cout<<"Transaction Successfully added.."<<endl;
    cout<<endl;
}

bool Blockchain::validateChain() const {
    // Traverse the chain and validate each block's hash
    Block* currentBlock = chainHead;
    while (currentBlock->previousBlock != nullptr) {
        //calculate hash of previous block
        TransactionNode* transactionhead = currentBlock->previousBlock->transactions;
        string transactions =  "";
        while(transactionhead != NULL) {
            transactions += transactionhead->transaction;
            transactionhead = transactionhead->next;
        }
        
        string prevBlockHash = calculateHash(transactions);
    
        if (currentBlock->previousHash != prevBlockHash) {
            return false; // Invalid chain
        }
        currentBlock = currentBlock->previousBlock;
    }

    return true; // return true if Valid chain
}




void Blockchain::printChain() const {
    Block* currentBlock = chainHead;
    while (currentBlock != nullptr) {
        cout << "Block Index: " << currentBlock->index << endl;
        cout << "Block Hash:  " << currentBlock->blockHash << endl;
        cout << "Previous Hash: " << currentBlock->previousHash << endl;

        cout << "Transactions:" << endl;
        TransactionNode* currentTransaction = currentBlock->transactions;
        while (currentTransaction != nullptr) {
            cout << "  - " << currentTransaction->transaction << endl;
            currentTransaction = currentTransaction->next;
        }

        cout << "------------------------" << endl;
        cout<<endl;
        currentBlock = currentBlock->previousBlock;
    }
}



string Blockchain::calculateHash(const string& data) const {
    EVP_MD_CTX* mdContext = EVP_MD_CTX_new();

    EVP_DigestInit(mdContext, EVP_sha256());
    EVP_DigestUpdate(mdContext, data.c_str(), data.length());

    unsigned char hash[EVP_MAX_MD_SIZE];
    unsigned int hashLength;
    EVP_DigestFinal(mdContext, hash, &hashLength);

    EVP_MD_CTX_free(mdContext);

    stringstream ss;
    ss << hex << setfill('0');
    for (unsigned int i = 0; i < hashLength; ++i) {
        ss << setw(2) << static_cast<int>(hash[i]);
    }

    return ss.str();
}



//main function
int main() {
    cout << "Starting Blockchain Ledger..." << endl;
    Blockchain blockchain;

    while (true) {
        int inputNumber;
        cout << "Enter a number to perform an operation:" << endl;
        cout << "1: Add Transaction" << endl;
        cout << "2: Add Block" << endl;
        cout << "3: Validate Ledger" << endl;
        cout << "4: Print Ledger" << endl;
        cout << "5: Exit" << endl;
        cin >> inputNumber;
        cout<<endl;

        switch (inputNumber) {
            case 1: {
                cout << "Enter Transaction: ";
                string transactionInput;
                cin.ignore(); 
                getline(cin, transactionInput);
                blockchain.addTransaction(transactionInput);
                break;
            }
            case 2:
                blockchain.addBlock(blockchain.transactionPool);
                break;
            case 3: {
                bool isValid = blockchain.validateChain();
                if (isValid) {
                    cout << "Blockchain is valid." << endl;
                } else {
                    cout << "Blockchain is invalid." << endl;
                }
                break;
            }
            case 4:
                blockchain.printChain();
                break;
            case 5:
                cout << "Exiting..." << endl;
                return 0;
            default:
                cout << "Error: Unknown operation." << endl;
                break;
        }
    }

    return 0;
}
