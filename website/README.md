# OpenFlightDeck – Landingpage

Statische Seite ohne Build-Schritt und ohne externe Requests (Schriften liegen in `fonts/`).
Zum Ansehen `index.html` im Browser öffnen oder lokal ausliefern, z. B. `python3 -m http.server` in diesem Ordner.

- `index.html` – Inhalt (Deutsch)
- `styles.css` – Gestaltung, Animationen
- `main.js` – animierte iPad-Karte (Illustration, keine echten Daten), Scrollytelling, Turbulenz-Spiel, Roadmap-Linie
- `assets/hero.jpg` – optionales Hintergrundfoto für den Hero. Fehlt es, zeigt die Seite einen gezeichneten Abendhimmel.
- `fonts/` – Bricolage Grotesque, Inter, JetBrains Mono (SIL Open Font License)

Gedacht für den Launch ab Phase 1 (App verfügbar). Offene TODOs (im HTML markiert): App-Store- bzw. TestFlight-Link, Impressum, Datenschutz. Die Karte im iPad ist eine Illustration und kann später durch echte Screenshots ersetzt werden.
`prefers-reduced-motion` wird respektiert.
