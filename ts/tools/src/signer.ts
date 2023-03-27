import { toBech32 } from "@cosmjs/encoding";
import { Hash, Mnemonic, PrivKeySecp256k1 } from "@keplr-wallet/crypto";

const [, , MNEMONIC] = process.argv;

const phrase =
  MNEMONIC ||
  "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";

console.log(`Input mnemonic: ${phrase}`);

async function main() {
  const bzToStr = (bz: Uint8Array) => Buffer.from(bz).toString("hex");

  const privKeyBz = Mnemonic.generateWalletFromMnemonic(phrase);
  const privKey = new PrivKeySecp256k1(privKeyBz);
  const pubKey = privKey.getPubKey();
  const addr = toBech32("osmo", pubKey.getAddress());

  const digest = Hash.sha256(Buffer.from(addr));
  const sign = privKey.signDigest32(digest);

  console.log("pubkey    :", bzToStr(pubKey.toBytes()));
  console.log("privkey   :", bzToStr(privKey.toBytes()));
  console.log("address   :", addr);
  console.log("signature :", bzToStr(sign));

  const verified = pubKey.verifyDigest32(digest, sign);
  console.log("verified  :", verified);
}

main().catch(console.error);
