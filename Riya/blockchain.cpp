/*-------BLOCK CHAIN PROGRAM USING C++-------*/

//headerfiles used in the program
#include <iostream>
#include <list>
#include <string>
#include <sstream>
#include <functional>

using namespace std;

//Transaction class used to hold details of each transaction
class Transaction {
    public:
    string sender;
    string receiver;
    float price;

    //constructor to initialize the transaction
    Transaction(string sender, string receiver, float price) {
        this->sender = sender;
        this->receiver = receiver;
        this->price = price;
    }
};


//Block class used to calculate hash values and initialize the block
class Block {
    public:
    string prevhash; //stores hash of previous block
    string hash; //strores hash of current block
    list<Transaction> transaction; //list of transactions
    Block* prev; //link to previous block
    Block* next; //link to next block
    
    private:
    //function to calculate a simple hash value using hash class
    string calcHash() const {
        stringstream ss;
        ss << prevhash;

        for (const auto& tx : transaction) {
            ss << tx.sender << tx.receiver << tx.price;
        }

        //hash class object created
        std::hash<string> hasher;
        return to_string(hasher(ss.str()));
    }
    
    public:
    //constructor to initialize the block
    Block(const std::string& prevhash, const list<Transaction>& transaction) {
        this->prevhash = prevhash;
        this->transaction = transaction;
        this->prev = nullptr;
        this->next = nullptr;
        hash = calcHash();
    }
};

//Blockchain class to create the blocks, validate them and link the blocks
class Blockchain {
    Block* head;
    
    public:
    //constructor to intialize the genesis block
    Blockchain() {
        head = new Block("0",list<Transaction>());
    }

    //function to get the last block created
    Block* getLatestBlock() const {
        Block* current = head;
        while(current->next != nullptr) {
            current = current->next;
        }
        return current;
    }

    //function to get the head block
    Block* getHeadBlock() const {
        return head;
    }

    //function to add new blocks
    void addBlock(const list<Transaction>& transaction) {
        Block* prevBlock = getLatestBlock();
        Block* newBlock = new Block(prevBlock->hash,transaction);
        prevBlock->next = newBlock;
    }

    //function to validate the blockchain
    bool validateChain() const {
        Block* previous = head;
        Block* current = head->next;

        while(current->next != nullptr) {
            if (current->prevhash != previous->hash) {
                return false;
            }

            previous = previous->next;
            current = current->next;
        }
        
        return true;
    }
};

//function to add transaction for each block
void addTransactions(Blockchain& blockchain, list<Transaction> transactions) {
    int n;
    string namesend, namerec;
    float price;

    cout << "\nEnter number of transactions : ";
    cin >> n;

    for (int i = 0; i < n; i++) {
        cout << "\nEnter name of sender " << i + 1 << " : ";
        cin >> namesend;

        cout << "Enter name of receiver " << i + 1 << " : ";
        cin >> namerec;

        cout << "Enter the cost of transaction " << i + 1 << " : ";
        cin >> price;

        transactions.push_back(Transaction(namesend, namerec, price));
    }

    blockchain.addBlock(transactions);
    cout << "\n\nAll transactions have been added to the Block!!";
}

//fucntion to view the blockchain
void viewBlockchain(const Blockchain& blockchain) {
    Block* current = blockchain.getHeadBlock();
    int blockCount = 0;

    //used to skip showing the genesis block
    current = current->next;

    while (current != nullptr) {
        blockCount++;

        cout << "\n\nBlock " << blockCount << " - Hash: " << current->hash << endl;
        cout << "Transactions:" << endl;

        for (const auto& tx : current->transaction) {
            cout << "  Sender: " << tx.sender << ", Receiver: " << tx.receiver << ", Price: " << tx.price << endl;
        }

        cout << endl;

        current = current->next;
    }
}

int main() {
    Blockchain blockchain;
    list<Transaction> transactions;
    int ch;

    while (true) {
        cout << "\n\n\n-------BLOCKCHAIN PROGRAM-------";
        cout << "\n\n1. Add Block\n2. View BlockChain\n3. Validate Blockchain\n4. Exit\n";
        cout << "\nEnter your choice : ";
        cin >> ch;

        switch (ch) {
            case 1:
                addTransactions(blockchain,transactions);
                break;

            case 2:
                viewBlockchain(blockchain);
                break;
            
            case 3:
                if(blockchain.validateChain())
                    cout<<"\nThe traveresing is done!! The Blockchain is secure";
                else
                    cout<<"\nError in hash values!! The Blockchain is not secure";
                break;

            case 4:
                exit(0);
                break;

            default:
                cout << "Invalid Choice";
                exit(0);
        }
    }

    return 0;
}
