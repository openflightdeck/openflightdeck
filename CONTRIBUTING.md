# Beitragen zu OpenFlightDeck

Danke, dass du mithelfen willst. Dieses Dokument beschreibt, wie Beiträge ablaufen; die technischen Regeln stehen in [AGENTS.md](AGENTS.md) und gelten für Menschen genauso wie für Coding-Agenten.

## Developer Certificate of Origin (DCO)

Wir nutzen das [Developer Certificate of Origin 1.1](DCO) statt eines CLA. Mit deinem Sign-off bestätigst du, dass du das Recht hast, den Beitrag unter der Lizenz der betroffenen Dateien einzubringen.

Jeder Commit braucht eine Zeile `Signed-off-by` mit deinem echten Namen und deiner E-Mail-Adresse:

```
Signed-off-by: Erika Mustermann <erika@example.org>
```

Am einfachsten mit `git commit -s`. Fehlt der Sign-off, schlägt die DCO-Prüfung im Pull Request fehl; nachholen lässt er sich mit `git rebase --signoff main` und anschließendem `git push --force-with-lease`.

## Lizenz deines Beitrags

Ein Beitrag steht unter der Lizenz der Crate oder des Ordners, in dem er landet: MIT oder Apache-2.0 für die Bibliotheks-Crates, MPL-2.0 für alles andere (siehe Tabelle im [README](README.md#lizenz)). Bei Apache-2.0 gilt Abschnitt 5 der Lizenz: Beiträge werden ohne Zusatzbedingungen unter derselben Lizenz eingebracht.

## Ablauf

1. Lies `docs/STATUS.md` und such dir eine offene Zeile in `docs/parity.md` oder ein Issue.
2. Ein Branch pro Aufgabe, von `main` abgezweigt.
3. `cargo xtask ci` muss lokal grün sein, bevor du einen Pull Request öffnest.
4. Fehlerbehebungen kommen mit einem Regressionstest, der vorher rot war.
5. Änderungen an `ofd-units`, `ofd-geo` und `ofd-alerts` brauchen immer ein Review durch einen Maintainer, weil dort fliegerisch kritische Logik liegt.

## Daten und Fixtures

Fixtures unter `fixtures/` gelten als Wahrheit für Tests. Sie werden erst aufgenommen, wenn ein Mensch sie gegen die amtliche ICAO-Karte geprüft hat; vermerke im Pull Request, womit und mit welchem Stand du geprüft hast.
