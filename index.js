let web3 = require('@solana/web3.js');
let splToken = require('@solana/spl-token');

let firstWinPrivKey = [162,198,104,103,207,4,235,232,23,248,93,95,16,210,192,64,64,173,226,15,71,204,75,177,142,211,87,151,56,187,21,215,206,55,206,167,210,17,110,45,138,212,214,46,106,204,205,123,251,83,72,89,237,245,78,33,0,95,187,91,225,2,166,8].slice(0,32);

  console.log(firstWinPrivKey.length);
  let firstWinWallet = web3.Keypair.fromSeed(Uint8Array.from(firstWinPrivKey));
  console.log(firstWinWallet.secretKey.toString());
  console.log(firstWinWallet.publicKey.toString());