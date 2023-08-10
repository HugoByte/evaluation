import java.util.List;
import java.util.LinkedList;
import java.security.NoSuchAlgorithmException;
import java.security.MessageDigest;

class Block 
{
    private List<String> transactions;
    private String previousHash;
    private String hash;

    public Block(List<String> transactions, String previousHash) 
    {
        this.transactions = transactions;
        this.previousHash = previousHash;
        this.hash = computeHash();
    }

    public String computeHash()
    {
        try 
        {
            MessageDigest md = MessageDigest.getInstance("SHA-256");
            String data = transactions.toString() + previousHash;
            byte[] hashBytes = md.digest(data.getBytes());
            StringBuilder hexString = new StringBuilder();

            for (byte hashByte : hashBytes) 
            {
                String hexDecimal = Integer.toHexString(0xff & hashByte);
                if (hexDecimal.length() == 1)
                {
                	hexString.append('0');
                }
                hexString.append(hexDecimal);
            }
            return hexString.toString();
        }
        catch (NoSuchAlgorithmException e)
        {
            e.printStackTrace();
            return null;
        }
    }

    public String getHash() 
    {
        return hash;
    }

    public String getPreviousHash()
    {
        return previousHash;
    }

    public List<String> getTransactions()
    {
        return transactions;
    }
}

class Blockchain 
{
    private List<Block> chain;                                        

    public Blockchain()                                          		// Executed when a new Blockchain object is created.
    {
        chain = new LinkedList<>();                                 
        chain.add(createGenesisBlock());                            
	/* The createGenesisBlock() method is called to generate the first block (also known as the "genesis block"), which contains an 	empty list of transactions and a previous hash value of "0". This block is then added to the chain. */
    }

    private Block createGenesisBlock()   //Create and return the genesis block. This block serves as the starting point of the blockchain.
    {
        return new Block(new LinkedList<>(), "0");
    }

    // Adds a new block to the chain. It takes a list of transactions as input.
    public void addBlock(List<String> transactions)                           
    {
        String previousHash = chain.get(chain.size() - 1).getHash();          // Retrieves the hash of the last block in the chain.
        Block newBlock = new Block(transactions, previousHash);               
        chain.add(newBlock);                                                  // The new block is added to the chain.
        //System.out.println(newBlock.getHash());
    }

    public boolean validate() 
    {
        for (int i = 1; i < chain.size(); i++) 
        {
            Block currentBlock = chain.get(i);
            Block previousBlock = chain.get(i - 1);

            if (!currentBlock.getHash().equals(currentBlock.computeHash())) 
            {
                System.out.println("Sorry! Block hash does not match its content.");
                return false;
            }

            if (!currentBlock.getPreviousHash().equals(previousBlock.getHash()))
            {
                System.out.println("Oops! Chain is Broken.");
                return false;
            }
        }
        System.out.println("Great! Blockchain is Valid.");
        return true;
    }
}

public class HugoByte
{
    public static void main(String[] args)
    {
        // Create a blockchain
        Blockchain b = new Blockchain();

        // Add some transactions to test the chain
        List<String> transactions1 = new LinkedList<>();
        transactions1.add("Alice sent Rs 100 as her savings to Bob.");
        transactions1.add("Bob invested Alice's Rs 100 in Mutual Funds.");
        b.addBlock(transactions1);

        List<String> transactions2 = new LinkedList<>();
        transactions2.add("Bob got Rs 150 back as Returns from Mutual Funds.");
        transactions2.add("Bob returned Rs 100 back to Alice retaining the profit he got.");
        b.addBlock(transactions2);

        // Validate the blockchain
        b.validate();
    }
}