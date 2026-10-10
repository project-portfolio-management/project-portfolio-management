#!/bin/sh
# Print a PUBLIC-ONLY Ed25519 key set (the RFC 8032 section 7.1 test vector 1
# public key) in the shape PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS expects.
#
# It lets a throwaway stack (CI smoke tests, a first `docker compose up`) boot
# with login enforcement ON: the service can verify tokens signed by the
# matching private key, which is public test material, so NEVER use this
# anywhere that matters. Real deployments publish their auth service's keys.
#
#   PPM_PASETO_KEYS="$(scripts/smoke-keys.sh)" POSTGRES_PASSWORD=x docker compose up -d --build --wait
set -eu
python3 - <<'PY'
import base64, hashlib, json
public = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
b64 = lambda b: base64.urlsafe_b64encode(b).rstrip(b"=").decode()
print(json.dumps({"keys": [{"kty": "OKP", "crv": "Ed25519", "use": "sig",
                            "kid": b64(hashlib.sha256(public).digest()), "x": b64(public)}]}))
PY
