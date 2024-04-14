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

Start DB:

```sh
surreal start --strict memory
```

Or via Surrealist

First time-init:

```pwsh
.\database\startup.ps1 root root
```

```sh
curl -X POST -H "Accept: application/json" -d '{"ns":"accounts", "db":"dev", "sc":"user", "username":"Tobias", "email":"t@h.de", "password":"1234"}' http://localhost:8000/signup
curl -X POST -H "Accept: application/json" -H "namespace: accounts" -H "database: dev" -H "scope: user" -d '{"username":"Tobias","email":"t@h.de","pass":"1234"}' http://localhost:8000/signup
```
