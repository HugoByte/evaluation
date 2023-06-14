#ifndef BLOCKCHAIN_H
#define BLOCKCHAIN_H

#include <string>
#include "block.h"
#include "transaction.h"

class Blockchain {
private:
    Block* chainHead;
public:
    TransactionNode* transactionPool;
    Blockchain();
    ~Blockchain();
    void addBlock(TransactionNode* transactions);
    void addTransaction(const std::string& transaction);
    bool validateChain() const;
    void printChain() const;

private:
    std::string calculateHash(const std::string& data) const;
};

#endif // BLOCKCHAIN_H
