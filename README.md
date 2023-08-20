## Ed 25519 

This is an experiment to verifying ed22519 in Ethereum. 

This has alot of issues that need to be resolved

- Invariant Testing: The idea to to use invariant testing to compare the signatures verified in rust with the solidity verison
- Hashing Algo : ed25519 uses SHA-512 as the hashing algorithm. So this needs to be replaced. We could probably use this : https://github.com/SmartPool/contracts/blob/develop/contracts/Ethash.sol 
- It is also very expensive even with our 50 M blocks . Each scalar mul costs 1.25 M

