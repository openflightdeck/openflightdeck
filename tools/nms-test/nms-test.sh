#!/usr/bin/env bash
# OpenFlightDeck – NMS-Staging-Test: Liefert NMS deutsche NOTAMs?
# Zugangsdaten werden NUR lokal gelesen und nie gespeichert oder ausgegeben.
# Vorbereitung (einmalig):
#   mkdir -p ~/.config/openflightdeck && chmod 700 ~/.config/openflightdeck
#   Datei ~/.config/openflightdeck/nms.env anlegen mit zwei Zeilen:
#     NMS_CLIENT_ID=<KEY aus der Excel-Datei>
#     NMS_CLIENT_SECRET=<SECRET aus der Excel-Datei>
#   chmod 600 ~/.config/openflightdeck/nms.env
# Aufruf: bash nms-test.sh   -> Ergebnis in nms-test-ergebnis.txt (ohne Zugangsdaten)
set -euo pipefail

ENV_FILE="${NMS_ENV_FILE:-$HOME/.config/openflightdeck/nms.env}"
[ -f "$ENV_FILE" ] || { echo "Fehlt: $ENV_FILE (siehe Kopf des Skripts)"; exit 1; }
# shellcheck disable=SC1090
source "$ENV_FILE"
: "${NMS_CLIENT_ID:?NMS_CLIENT_ID fehlt in $ENV_FILE}"
: "${NMS_CLIENT_SECRET:?NMS_CLIENT_SECRET fehlt in $ENV_FILE}"

HOST="https://api-staging.cgifederal-aim.com"
API="$HOST/nmsapi/v1"
OUT="$(dirname "$0")/nms-test-ergebnis.txt"
TMP="$(mktemp -d)"; trap 'rm -rf "$TMP"' EXIT

log() { echo "$*" | tee -a "$OUT"; }
: > "$OUT"
log "NMS-Staging-Test $(date -u +%Y-%m-%dT%H:%M:%SZ)"

# 1) Token holen (Secret geht nur an den Auth-Endpunkt, wird nicht protokolliert)
TOKEN_JSON="$(curl -sS -X POST "$HOST/v1/auth/token" -d grant_type=client_credentials -u "$NMS_CLIENT_ID:$NMS_CLIENT_SECRET")"
TOKEN="$(printf '%s' "$TOKEN_JSON" | sed -n 's/.*"access_token"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p')"
[ -n "$TOKEN" ] || { log "Token-Abruf fehlgeschlagen (Antwort nicht protokolliert)."; exit 1; }
log "Token: ok"

get() { # $1 = Name, $2 = Pfad+Query, $3 = optional Format-Header
  local name="$1" path="$2" fmt="${3:-}"
  local hdr=(-H "Authorization: Bearer $TOKEN")
  [ -n "$fmt" ] && hdr+=(-H "nmsResponseFormat: $fmt")
  local code
  code="$(curl -sS -o "$TMP/$name.json" -w '%{http_code}' "${hdr[@]}" "$API$path")"
  local size; size="$(wc -c < "$TMP/$name.json" | tr -d ' ')"
  log ""
  log "== $name  ($path)  HTTP $code, $size Bytes"
  head -c 1500 "$TMP/$name.json" | tee -a "$OUT" >/dev/null; echo >> "$OUT"
}

get ping        "/ping"
get eddf        "/notams?location=EDDF" GEOJSON
get edny        "/notams?location=EDNY" GEOJSON
get fir_edgg    "/notams?location=EDGG" GEOJSON
get geo_eddf    "/notams?latitude=50.033&longitude=8.570&radius=25" GEOJSON
get check_eddf  "/notams/checklist?location=EDDF"
get check_intl  "/notams/checklist?classification=INTERNATIONAL"
get kdfw_ref    "/notams?location=KDFW" GEOJSON

# 2) Initial Load INTERNATIONAL: kommen deutsche Orte (ED..) vor?
log ""
log "== Initial Load INTERNATIONAL"
if curl -sS -L -o "$TMP/intl.gz" -H "Authorization: Bearer $TOKEN" "$API/notams/il/INTERNATIONAL"; then
  log "Datei: $(wc -c < "$TMP/intl.gz" | tr -d ' ') Bytes"
  if gzip -t "$TMP/intl.gz" 2>/dev/null; then
    gzip -dc "$TMP/intl.gz" > "$TMP/intl.xml"
    log "NOTAMs gesamt (AIXMBasicMessage): $(grep -o '<AIXMBasicMessage' "$TMP/intl.xml" | wc -l | tr -d ' ')"
    log "Häufigste ICAO-Präfixe (2 Buchstaben):"
    grep -oE '>[A-Z]{4}<' "$TMP/intl.xml" | cut -c2-3 | sort | uniq -c | sort -rn | head -15 | tee -a "$OUT" >/dev/null
    log "Treffer für ED..-Orte:"
    grep -oE '>ED[A-Z]{2}<' "$TMP/intl.xml" | sort | uniq -c | sort -rn | head -15 | tee -a "$OUT" >/dev/null
  else
    log "Antwort ist kein gzip, Anfang:"; head -c 800 "$TMP/intl.gz" >> "$OUT"
  fi
else
  log "Download fehlgeschlagen."
fi

log ""
log "Fertig. Bitte nms-test-ergebnis.txt zurückschicken (enthält keine Zugangsdaten)."
