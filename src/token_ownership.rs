the token program assign each token account an owner but is not the same as the account owner. 
this means that solana program will always be the account owner. The token account will be only 
accept instructions from the token owner which is declared by the token program and encoded inside the token 
account properties. 
once the token account is made , its private key is useless and only the token owner attribute matters.
the address of this token will become the main address of the user/owner account.

they only need their main private key to sign the tx to transact.

