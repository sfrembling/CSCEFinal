## Dialogue
Dialogue actions should be written as if it was English.

* You can use `entity`, `item`, and `location` tags.
* Example: `Please help! I can't find my {item}!`

## Objectives
Objective actions are only passed as commands.
The commands get parsed and each one generates some
english text.

The following commands are available:

| Command | Dialogue equivalent | Usage |
| --- | --- | --- |
| GOTO | Go to {location} | GOTO
| FIND | Find {item} | FIND
| FINDN | Find 5 {item}s | FINDN {number}
| KILL | Kill {entity} | KILL
| KILLN | Kill 13 {entities} | KILL {number}
| RET | Return here | RET
| LOOT | Take the {item} | LOOT
| LOOTN | Take 17 {item}s | LOOTN {number}
| GUARD | Guard the area | GUARD

When writing commands, use all caps, and separate them with a semicolon, like here (notice the last command doesn't need a semicolon):  
`GOTO;FINDN {12};RET`

That will generate something like:
```
1. Go to Ravenholm
2. Find 12 knifes
3. Return here
```
