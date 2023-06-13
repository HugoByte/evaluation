#include <iostream>
#include <string>
#include <vector>
#include <ctime>

class Transaction
{
public:
    std::string from;
    std::string to;
    double amount;

    Transaction(const std::string &from, const std::string &to, double amount)
        : from(from), to(to), amount(amount) {}
};

class Block
{
public:
    std::string hash;
    std::string previousHash;
    std::vector<Transaction> transactions;

    Block(const std::string &previousHash, const std::vector<Transaction> &transactions)
        : previousHash(previousHash), transactions(transactions)
    {
        hash = calculateHash();
    }

    std::string calculateHash() const
    {
        std::hash<std::string> hashFunc;
        std::string data = previousHash;
        for (const Transaction &transaction : transactions)
        {
            data += transaction.from + transaction.to + std::to_string(transaction.amount);
        }
        std::time_t timestamp = std::time(nullptr);
        data += std::asctime(std::localtime(&timestamp));

        return std::to_string(hashFunc(data));
    }
};

class Blockchain
{
public:
    std::vector<Block> chain;

    Blockchain()
    {
        std::vector<Transaction> emptyTransactions;

        Block genesisBlock("0", emptyTransactions);
        chain.push_back(genesisBlock);
    }

    void addBlock(const std::vector<Transaction> &transactions)
    {
        const std::string &previousHash = chain.back().hash;

        Block newBlock(previousHash, transactions);
        chain.push_back(newBlock);
    }

    bool validateChain() const
    {
        for (size_t i = 1; i < chain.size(); ++i)
        {
            const Block &currentBlock = chain[i];
            const Block &previousBlock = chain[i - 1];

            if (currentBlock.hash != currentBlock.calculateHash())
            {
                return false;
            }

            if (currentBlock.previousHash != previousBlock.hash)
            {
                return false;
            }
        }
        return true;
    }

    void printChain() const
    {
        for (const Block &block : chain)
        {
            std::cout << "Block Hash: " << block.hash << std::endl;
            std::cout << "Previous Hash: " << block.previousHash << std::endl;
            std::cout << "Transactions: " << std::endl;
            for (const Transaction &transaction : block.transactions)
            {
                std::cout << "-> From: " << transaction.from << std::endl;
                std::cout << "-> To: " << transaction.to << std::endl;
                std::cout << "-> Amount: " << transaction.amount << std::endl;
            }
            std::cout << std::endl;
        }
    }
};

int main()
{
    // Create a blockchain
    Blockchain blockchain;

    // Add transactions
    std::vector<Transaction> transactions1;
    transactions1.push_back(Transaction("Rupen", "Rushi", 10.0));
    blockchain.addBlock(transactions1);

    std::vector<Transaction> transactions2;
    transactions2.push_back(Transaction("Rushi", "Paavan", 5.0));
    blockchain.addBlock(transactions2);

    std::vector<Transaction> transactions3;
    transactions3.push_back(Transaction("Paavan", "Rupen", 7.0));
    blockchain.addBlock(transactions3);

    // Print the chain
    blockchain.printChain();

    // Validate the chain
    bool isValid = blockchain.validateChain();
    std::cout << "Chain is valid? " << (isValid ? "Yes" : "No") << std::endl;

    return 0;
}