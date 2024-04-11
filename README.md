# Raffle Drop
## Description
This contract is simple raffle drop contract. It allows creator to create a raffle drop with a specific number of participants. Every participant is represented with an id. 

## Instantiate
To instantiate the contract, the creator must provide the number of participants and nois proxy address. The nois proxy address is used to get randomness from the nois proxy contract.

## Functions

### Request Randomness
This function is used to request random number from the nois proxy contract. Only the creator can call this function. The creator must provide job id along with the funds to request randomness.

### Nois Receive
This function is called by the nois proxy contract to send the random number to the raffle drop contract. The random number is used to select the winners of the raffle drop. Proxy contract also return job_id along with the randomness. If job_id is "test" then the contract will select test winners and will save it under test winners.