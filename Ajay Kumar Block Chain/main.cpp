#include<iostream>
#include<list>
#include "transaction.h"
#include "block.h"
#include "blockchain.h"

int main(){

  BlockChain blockChain;

  // New list of transactions
  std::list<Transaction> transaction1=
  {
    Transaction("User1","User2",2000),
    Transaction("User2","User3",1500),
    Transaction("User3","User4",2700)
  };
  
  // Add new Block
  blockChain.addBlock(transaction1);
  
  // another list
  std::list<Transaction> transaction2=
  {
    Transaction("User1","User2",4000),
    Transaction("User2","User3",8600),
    Transaction("User3","User4",7300)
  };
  blockChain.addBlock(transaction2);

  std::list<Transaction> transaction3=
  {
    Transaction("User4","User5",8000),
    Transaction("User4","User6",2500),
    Transaction("User4","User7",6300)
  };

  std::list<Transaction> transaction4=
  {
    Transaction("User1","User2",6000),
    Transaction("User2","User3",3500),
    Transaction("User3","User4",4500)
  };

  std::list<Transaction> transaction5=
  {
    Transaction("User1","User2",1000),
    Transaction("User2","User3",9000),
    Transaction("User3","User4",2300)
  };

  // adding blocks
  blockChain.addBlock(transaction3);
  blockChain.addBlock(transaction4);
  blockChain.addBlock(transaction5);

  blockChain.printChain();
  blockChain.validationReport();

  return 0;
}   