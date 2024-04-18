# My-Passwords

Roadmap:

- Offline Mode
- Fix Accessibility
- Fetching personal secret from file
- Buckets: e.g. Personal, IBM, DHBW, FU etc... + Ability to Move Acc between Buckets
- Automaticly search for institution website and write it to acc-page
- SSO: Show which SSO Acc was used, if none show password. Option to let account be registred as an SSO -> Own Settings Page where User can select Acc to Link to SSO
- Note of Authenticator Usage
- Fields for Recovery Codes, Security Word etc.
- Aliases for search. E.g. if a company is mostly known for product A (where you login) but the company itself is actually called B
- Change database either to surrealdb for the flex or make a git-backed file-system password store for [password history](https://www.youtube.com/watch?v=FhwsfH2TpFA)
- SuperSecure Mode: Monatsabhänig machen für z.B. IBM wo alle 3 Monate geupdatet werden muss
- Passwort mit freier Eingabe
- Sanity Checks: Überprüfen, ob Acc / Bucket / Institution zum Benutzer gehört
- Add indicies to DB

## Database Stuff

Start DB in memory

```sh
surreal start --strict memory
```

*or via Surrealist.*

First time-init:

```pwsh
cd .\database\
cargo run
```

## Roadmap

- [x] Create Secure
- [x] Secure Password Site
- [x] Edit Secure
- [x] Delete Secure
- [x] Store PIN and SECRET on local
- [x] Store Special-Chars in DB
- [x] For Super-Secure-Case: Replace "year" with "seed" -> must be equal to seed = year%3 so that seed%3
- [x] Super Secure Algo
- [x] Create Super Secure
- [x] Super Secure Password Site
- [x] Create SSO
- [x] SSO Password Site
- [x] Legacy Password Site
- [x] List of all Passwords
- [ ] Store Recent Searched / Popular Searches
- [x] Create Buckets
- [x] Delete Buckets
- [ ] Manage Buckets
- [ ] Dynamic Username Top left corner
- [ ] Wallpaper Credits auf Logic Page und in pot. Footer
- [ ] Input Sanitation
- [ ] Reorga main page -> Create Bucket must be possible from there
- [ ] Run Clippy and remove all println / dbg / console.log and all other warnings
- [ ] Version 2
  - [ ] Create 2FA
  - [ ] Manage 2FA
  - [ ] Dashboard with Statistics
  - [ ] Create Hand-Written Passwords
  - [ ] Random Bucket Wallpaper
  - [ ] Custom Theme builder einbauen & themes local speichern
  - [ ] Options Page
  - [ ] Drag n Drop resorting of accounts into Buckets on the List Page
