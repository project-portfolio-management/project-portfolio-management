// A PASETO v4.public token for the demonstration person, signed with a PUBLISHED test key.
//
// The demonstration API runs with login ON (so `/api/me/tasks`, saved views and the rest know who is asking) and
// trusts exactly one public key, whose private half is the constant seed below. That seed is public on purpose:
// anyone can mint a token for a demonstration stack, which is why the demonstration holds only fictitious data and
// must never share a key set, a network or a database with anything real (`operations/demo.md`).

import {
  createHash,
  createPrivateKey,
  createPublicKey,
  sign,
} from "node:crypto";

/** The Ed25519 seed of the demonstration key (32 bytes of 0x44). Public test material. */
export const DEMO_SEED = Buffer.alloc(32, 0x44);

import { DEMO_SUB } from "../demo";

export { DEMO_SUB };

const ISSUER = "authentication-service";
const AUDIENCE = "main-x-service";

const b64url = (b: Buffer | Uint8Array): string =>
  Buffer.from(b).toString("base64url");

function keys() {
  const der = Buffer.concat([
    Buffer.from("302e020100300506032b657004220420", "hex"),
    DEMO_SEED,
  ]);
  const privateKey = createPrivateKey({
    key: der,
    format: "der",
    type: "pkcs8",
  });
  const publicBytes = createPublicKey(privateKey)
    .export({ format: "der", type: "spki" })
    .subarray(-32);
  return {
    privateKey,
    publicBytes,
    kid: b64url(createHash("sha256").update(publicBytes).digest()),
  };
}

/** The key set the demonstration API trusts (`PPM_PASETO_KEYS`). */
export function demoKeySet(): {
  keys: { kty: string; crv: string; use: string; kid: string; x: string }[];
} {
  const { publicBytes, kid } = keys();
  return {
    keys: [
      { kty: "OKP", crv: "Ed25519", use: "sig", kid, x: b64url(publicBytes) },
    ],
  };
}

/** PASETO pre-authentication encoding. */
export function pae(...pieces: Buffer[]): Buffer {
  const le64 = (n: number) => {
    const b = Buffer.alloc(8);
    b.writeBigUInt64LE(BigInt(n));
    return b;
  };
  return Buffer.concat([
    le64(pieces.length),
    ...pieces.flatMap((p) => [le64(p.length), p]),
  ]);
}

/**
 * A token for `example@example.com`. `attrs` carry the access the demonstration grants: writing, and the
 * administrator views (email deliveries). `nowSeconds` and `lifetimeSeconds` are injectable for tests.
 */
export function mintDemoToken(
  nowSeconds = Math.floor(Date.now() / 1000),
  lifetimeSeconds = 8 * 3600,
): string {
  const { privateKey, kid } = keys();
  const payload = Buffer.from(
    JSON.stringify({
      sub: DEMO_SUB,
      email: "example@example.com",
      name: "Example Person",
      iss: ISSUER,
      aud: AUDIENCE,
      iat: nowSeconds,
      exp: nowSeconds + lifetimeSeconds,
      sid: "demo-session",
      attrs: {
        access: ["write", "admin"],
        cutover_role: ["accountable_professional_lead"],
      },
    }),
  );
  const footer = Buffer.from(JSON.stringify({ kid }));
  const header = Buffer.from("v4.public.");
  const signature = sign(
    null,
    pae(header, payload, footer, Buffer.alloc(0)),
    privateKey,
  );
  return `v4.public.${b64url(Buffer.concat([payload, signature]))}.${b64url(footer)}`;
}
