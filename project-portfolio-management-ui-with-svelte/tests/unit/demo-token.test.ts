// The demonstration token: a well-formed PASETO v4.public that verifies against the published key set, and the
// key set in compose.yaml is that same key (so the demonstration API trusts what the UI signs).

import { describe, expect, it } from "vitest";
import { createPublicKey, verify } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import {
  DEMO_SUB,
  demoKeySet,
  mintDemoToken,
  pae,
} from "../../src/lib/server/demo-token";
// @ts-expect-error a plain ES module with no types
import {
  mintDemoToken as mintForSeed,
  DEMO_SUB as SEED_SUB,
} from "../../../scripts/demo/token.mjs";

const fromB64url = (s: string) => Buffer.from(s, "base64url");

describe("the demonstration token", () => {
  const token = mintDemoToken(1_000_000, 3600);

  it("is a v4.public token with a key-id footer", () => {
    const [v, purpose, body, footer] = token.split(".");
    expect(`${v}.${purpose}`).toBe("v4.public");
    expect(body!.length).toBeGreaterThan(100);
    expect(JSON.parse(fromB64url(footer!).toString()).kid).toBe(
      demoKeySet().keys[0]!.kid,
    );
  });

  it("verifies against the published public key", () => {
    const [, , body, footer] = token.split(".");
    const raw = fromB64url(body!);
    const message = raw.subarray(0, raw.length - 64);
    const signature = raw.subarray(raw.length - 64);
    const x = demoKeySet().keys[0]!.x;
    const spki = Buffer.concat([
      Buffer.from("302a300506032b6570032100", "hex"),
      fromB64url(x),
    ]);
    const publicKey = createPublicKey({
      key: spki,
      format: "der",
      type: "spki",
    });
    const ok = verify(
      null,
      pae(
        Buffer.from("v4.public."),
        message,
        fromB64url(footer!),
        Buffer.alloc(0),
      ),
      publicKey,
      signature,
    );
    expect(ok).toBe(true);
    const claims = JSON.parse(message.toString());
    expect(claims.sub).toBe(DEMO_SUB);
    expect(claims.email).toBe("example@example.com");
    expect(claims.iss).toBe("authentication-service");
    expect(claims.aud).toBe("main-x-service");
    expect(claims.exp - claims.iat).toBe(3600);
    expect(claims.attrs.access).toEqual(["write", "admin"]);
    expect(claims.attrs.cutover_role).toEqual([
      "accountable_professional_lead",
    ]);
  });

  it("is the key the compose demonstration API trusts", () => {
    const compose = readFileSync(
      resolve(process.cwd(), "../compose.yaml"),
      "utf8",
    );
    const { kid, x } = demoKeySet().keys[0]!;
    expect(compose).toContain(kid);
    expect(compose).toContain(x);
  });

  it("is the very token the seed job signs with (the two copies of the signer must not drift)", () => {
    expect(mintForSeed(1_000_000, 3600)).toBe(mintDemoToken(1_000_000, 3600));
    expect(SEED_SUB).toBe(DEMO_SUB);
  });

  it("pae follows the PASETO encoding", () => {
    expect(pae().toString("hex")).toBe("0000000000000000");
    expect(pae(Buffer.from("test")).toString("hex")).toBe(
      "0100000000000000" + "0400000000000000" + "74657374",
    );
  });
});
