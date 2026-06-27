#!/usr/bin/env bash
# scripts/validate.sh — Post-deploy validation for AkurAI-CRM
# Canonical pattern from AkurAI-Framework/scripts/validate.sh
set -euo pipefail

: "${DOMAIN:=akurai-crm.olibuijr.com}"
: "${PORT:=8103}"
: "${APP_NAME:=akurai-crm}"
RED='\033[0;31m'; GRN='\033[0;32m'; NC='\033[0m'
pass=0; fail=0
pass_() { printf "  ${GRN}PASS${NC} %s\n" "$*"; ((pass++)); }
fail_() { printf "  ${RED}FAIL${NC} %s\n" "$*"; ((fail++)); }

echo "=== Post-deploy validation: ${APP_NAME} ==="

# 1. Systemd
systemctl is-active --quiet "${APP_NAME}.service" 2>/dev/null && pass_ "systemd active" || fail_ "systemd not active"

# 2. Health
curl -fsS --max-time 5 "http://127.0.0.1:${PORT}/api/health" > /dev/null 2>&1 && pass_ "loopback health" || fail_ "loopback health"
curl -fsS --max-time 10 "https://${DOMAIN}/api/health" > /dev/null 2>&1 && pass_ "public health" || fail_ "public health"

# 3. Health JSON schema
HEALTH=$(curl -fsS --max-time 5 "https://${DOMAIN}/api/health" 2>/dev/null || echo '{}')
echo "$HEALTH" | python3 -c "
import json,sys
d=json.load(sys.stdin)
for k in ['app','version','status']:
    assert k in d, f'missing key: {k}'
assert d['status'] in ('ok','degraded','down'), f\"bad status: {d.get('status')}\"
" 2>/dev/null && pass_ "health JSON schema valid" || fail_ "health JSON schema invalid"

APP=$(echo "$HEALTH" | python3 -c "import json,sys; print(json.load(sys.stdin).get('app',''))" 2>/dev/null)
[ "$APP" = "${APP_NAME}" ] && pass_ "health app='${APP_NAME}'" || fail_ "health app='$APP' (expected '${APP_NAME}')"

# 4. OIDC login redirect → IDP
LOGIN_STATUS=$(curl -sS -o /dev/null -w '%{http_code}' --max-time 10 "https://${DOMAIN}/auth/login" 2>/dev/null)
if [ "$LOGIN_STATUS" = "302" ]; then
  # Verify redirect goes to auth.olibuijr.com
  LOCATION=$(curl -sI --max-time 10 "https://${DOMAIN}/auth/login" 2>/dev/null | grep -i '^location:' | awk '{print $2}' | tr -d '\r')
  if echo "$LOCATION" | grep -q 'auth.olibuijr.com'; then
    pass_ "auth/login → IDP redirect (correct)"
  else
    fail_ "auth/login → wrong redirect: $LOCATION"
  fi
else
  fail_ "auth/login → ${LOGIN_STATUS} (expected 302)"
fi

# 4. Landing page
curl -fsS --max-time 10 "https://${DOMAIN}/" 2>/dev/null | grep -q 'AkurAI-CRM\|CRM' && pass_ "landing page" || fail_ "landing page"

# 5. API _meta
curl -fsS --max-time 5 "https://${DOMAIN}/api/_meta" 2>/dev/null | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'routes' in d" 2>/dev/null && pass_ "_meta valid" || fail_ "_meta invalid"

echo "━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GRN}Pass: $pass${NC}  ${RED}Fail: $fail${NC}"
[ "$fail" -eq 0 ] || exit 1
