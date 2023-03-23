import { MerkleTree } from "merkletreejs";
import SHA256 from "crypto-js/sha256";
import { writeFileSync } from "fs";
import path from "path";

type InputData = [{ address: string; amount: string }] | string[];

const [airdropId, inputFile, outputFile] = process.argv.slice(2);

if (!airdropId || !inputFile || !outputFile) {
  console.log("Invalid arguments");
  process.exit(1);
}

console.log("Input: ");
console.log(` - Airdrop ID  : ${airdropId}`);
console.log(` - Input file  : ${inputFile}`);
console.log(` - Output file : ${outputFile}`);

const inputData: InputData = require(path.join(process.cwd(), inputFile));
const leaves =
  inputData[0] instanceof Object
    ? (inputData as { address: string; amount: string }[])
    : (inputData as string[]).map((amount) => ({
        address: SHA256(`${Math.random()}`).toString(),
        amount,
      }));

const tree = new MerkleTree(
  leaves.map(({ address, amount }) => SHA256(`${address}:${amount}`)),
  SHA256,
  { sort: true }
);

const root = tree.getHexRoot().replace("0x", "");

const proofs = leaves.map(({ address, amount }) => {
  const leaf = SHA256(`${address}:${amount}`);
  const proof = tree
    .getHexProof(leaf.toString())
    .map((v) => v.replace("0x", ""));

  return {
    address,
    amount,
    merkleRoot: root,
    merkleProof: proof,
  };
});

writeFileSync(
  path.join(process.cwd(), outputFile),
  JSON.stringify({ airdropId, proofs }, null, 2)
);

console.log("Done");
