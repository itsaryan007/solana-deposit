let web3 = require('@solana/web3.js');
let splToken = require('@solana/spl-token');

// load up the first 32 bytes of the 64 byte array that was in our keyfile.json
// Only need the first 32 bytes so I use slice() just to make sure it's the correct length
let firstWinPrivKey = [162,198,104,103,207,4,235,232,23,248,93,95,16,210,192,64,64,173,226,15,71,204,75,177,142,211,87,151,56,187,21,215,206,55,206,167,210,17,110,45,138,212,214,46,106,204,205,123,251,83,72,89,237,245,78,33,0,95,187,91,225,2,166,8].slice(0,32);
  // print the length of the array so we know it is correct
  // the fromSeed() method requires 32 bytes

  console.log(firstWinPrivKey.length);
  let firstWinWallet = web3.Keypair.fromSeed(Uint8Array.from(firstWinPrivKey));
  console.log(firstWinWallet.secretKey.toString());
  console.log(firstWinWallet.publicKey.toString());