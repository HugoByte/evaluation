package startproject;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.List;

class Transaction {
    String sender;
    String receiver;
    double amount;
    long timestamp;

    public Transaction(String sender, String receiver, double amount) {
        this.sender = sender;
        this.receiver = receiver;
        this.amount = amount;
        this.timestamp = System.currentTimeMillis();
    }
}

class Block {
    int indexValue;
    String previoushashKey;
    List<Transaction> transactions;
    String hashKey;
    long timestamp;

    public Block(int indexValue, String previoushashKey, List<Transaction> transactions) {
        this.indexValue = indexValue;
        this.previoushashKey = previoushashKey;
        this.transactions = transactions;
        this.timestamp = System.currentTimeMillis();
        this.hashKey = calculatehashKey();
    }

    String calculatehashKey() {
        try {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            String data = indexValue + previoushashKey + transactions.toString() + timestamp;
            byte[] hashKeyBytes = digest.digest(data.getBytes());

            StringBuilder hexString = new StringBuilder();
            for (byte hashKeyByte : hashKeyBytes) {
                String hex = Integer.toHexString(0xff & hashKeyByte);
                if (hex.length() == 1) hexString.append('0');
                hexString.append(hex);
            }

            return hexString.toString();
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
            return null;
        }
    }
}

class Blockchain {
    private List<Block> chain;

    public Blockchain() {
        chain = new ArrayList<>();
        chain.add(createGenesisBlock());
    }

    private Block createGenesisBlock() {
        return new Block(0, "0", new ArrayList<>());
    }

    public Block getLatestBlock() {
        return chain.get(chain.size() - 1);
    }

    public void addBlock(List<Transaction> transactions) {
        int indexValue = chain.size();
        String previoushashKey = getLatestBlock().hashKey;

        Block newBlock = new Block(indexValue, previoushashKey, transactions);
        chain.add(newBlock);
    }

    public boolean isValid() {
        for (int i = 1; i < chain.size(); i++) {
            Block currentBlock = chain.get(i);
            Block prevBlock = chain.get(i - 1);

            if (!currentBlock.hashKey.equals(currentBlock.calculatehashKey()) ||
                !currentBlock.previoushashKey.equals(prevBlock.hashKey)) {
                return false;
            }
        }
        return true;
    }
}

public class HugoByteBlockChain {

	public static void main(String[] args) {
        Blockchain blockchain = new Blockchain();

        List<Transaction> transactions = new ArrayList<>();
        transactions.add(new Transaction("Chaitra", "Harish", 20.0));
        transactions.add(new Transaction("Harish", "Sunil", 5.0));
        transactions.add(new Transaction("Sunil", "Chaitra", 3.0));

        blockchain.addBlock(transactions);

        if (blockchain.isValid()) {
            System.out.println("Blockchain is valid.");
        } else {
            System.out.println("Blockchain is not valid.");
        }
    }
}
