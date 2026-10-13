// A PASETO v4.public token for the demonstration person, signed with the published demonstration test key
// (the same one the UI signs with: project-portfolio-management-ui-with-svelte/src/lib/server/demo-token.ts,
// and a unit test keeps the two identical). The demonstration API trusts only this public key, so the seed job
// can write through an API that has login ON. The seed is public on purpose: this key must never protect anything real.

import { createHash, createPrivateKey, createPublicKey, sign } from "node:crypto";

const SEED = Buffer.alloc(32, 0x44);
export const DEMO_SUB = "00000000-0000-4000-8000-0000000000a0";
const b64url = (b) => Buffer.from(b).toString("base64url");

function keys() {
  const der = Buffer.concat([Buffer.from("302e020100300506032b657004220420", "hex"), SEED]);
  const privateKey = createPrivateKey({ key: der, format: "der", type: "pkcs8" });
  const publicBytes = createPublicKey(privateKey).export({ format: "der", type: "spki" }).subarray(-32);
  return { privateKey, kid: b64url(createHash("sha256").update(publicBytes).digest()) };
}

function pae(...pieces) {
  const le64 = (n) => {
    const b = Buffer.alloc(8);
    b.writeBigUInt64LE(BigInt(n));
    return b;
  };
  return Buffer.concat([le64(pieces.length), ...pieces.flatMap((p) => [le64(p.length), p])]);
}

export function mintDemoToken(nowSeconds = Math.floor(Date.now() / 1000), lifetimeSeconds = 8 * 3600) {
  const { privateKey, kid } = keys();
  const payload = Buffer.from(
    JSON.stringify({
      sub: DEMO_SUB, email: "example@example.com", name: "Example Person",
      iss: "authentication-service", aud: "main-x-service",
      iat: nowSeconds, exp: nowSeconds + lifetimeSeconds, sid: "demo-session",
      attrs: { access: ["write", "admin"], cutover_role: ["accountable_professional_lead"] },
    }),
  );
  const footer = Buffer.from(JSON.stringify({ kid }));
  const signature = sign(null, pae(Buffer.from("v4.public."), payload, footer, Buffer.alloc(0)), privateKey);
  return `v4.public.${b64url(Buffer.concat([payload, signature]))}.${b64url(footer)}`;
}
