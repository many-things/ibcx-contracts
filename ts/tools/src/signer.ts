import { toBech32 } from "@cosmjs/encoding";
import { Hash, Mnemonic, PrivKeySecp256k1 } from "@keplr-wallet/crypto";

const [, , MNEMONIC, TARGET] = process.argv;

if (!MNEMONIC) {
  throw Error("Please provide a mnemonic phrase");
}

if (!TARGET) {
  console.log(
    "INFO: No target is provided. The address will be used as the target."
  );
}

async function main() {
  const bzToStr = (bz: Uint8Array) => Buffer.from(bz).toString("hex");

  const privKeyBz = Mnemonic.generateWalletFromMnemonic(MNEMONIC);
  const privKey = new PrivKeySecp256k1(privKeyBz);
  const pubKey = privKey.getPubKey();
  const addr = toBech32("osmo", pubKey.getAddress());

  const digest = Hash.sha256(Buffer.from(TARGET || addr));
  const sign = privKey.signDigest32(digest);

  console.log(
    JSON.stringify(
      {
        pubKey: bzToStr(pubKey.toBytes()),
        privKey: bzToStr(privKey.toBytes()),
        address: addr,
        signature: bzToStr(sign),
      },
      null,
      2
    )
  );

  const verified = pubKey.verifyDigest32(digest, sign);
  if (!verified) {
    throw Error("verification failed");
  }
}

main().catch(console.error);
