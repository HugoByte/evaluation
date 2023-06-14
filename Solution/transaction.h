#ifndef TRANSACTION_H
#define TRANSACTION_H

#include <string>

struct TransactionNode {
    std::string transaction;
    TransactionNode* next;

    TransactionNode(const std::string& transaction) : transaction(transaction), next(nullptr) {}
};

#endif // TRANSACTION_H
