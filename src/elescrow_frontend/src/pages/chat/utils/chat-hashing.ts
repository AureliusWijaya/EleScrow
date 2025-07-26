import { Ed25519KeyIdentity } from "@dfinity/identity";
import { Principal } from "@dfinity/principal";
import { sha256 } from "js-sha256";

export const generateChatId = (
  userPrincipal: Principal,
  recipientPrincipal: Principal
): Ed25519KeyIdentity | null => {
  const seed = new TextEncoder().encode(
    userPrincipal.toText().concat(recipientPrincipal.toText())
  );
  const hashedSeed = sha256.create().update(seed).array();
  return Ed25519KeyIdentity.generate(Uint8Array.from(hashedSeed));
};
