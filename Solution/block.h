#ifndef BLOCK_H
#define BLOCK_H

#include <string>
#include "transaction.h"

struct Block {
    int index;
    std::string blockHash;
    std::string previousHash;
    TransactionNode* transactions;
    Block* previousBlock;

    Block() : transactions(nullptr), previousBlock(nullptr) {}
};

#endif // BLOCK_H
