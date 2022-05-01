# Procedural Quest Generator

by Shea Frembling

---

## How to Use

To compile: `cargo build --release`  
To run: `cargo run --release`

## Customizing Dialogue

In `data/dialogue/actions` is where you define an action, which is just spoken dialogue

You also place any tag-files in the `data/dialogue` directory

So for example, if I wanted to generate some random quests, I might do the following:

```
(in data/dialogue/actions)
Please, won't you help me find my {item}? The villains at {location} took it from me!

(I now need a corresponding file for item and location, so)

(in data/dialogue/item)
book
knife
money
purse
bag

(in data/dialogue/location)
Hogwarts
UNT
Boston
New York
Dallas
Caria Manor
```

## Customizing Objectives
Objectives are built up by a series of commands, with their definition being placed in `data/objectives/command_dict`

An example of some basic commands is as follows:

```
(in data/objectives/command_dict)
GOTO # Go to {location}
RET # Return here
KILL {} # Kill {} {enemy}s
```

Objectives do the same tag-replacement as dialogue does, the only difference being that it is placed into a numbered list. So any tag you use must be placed in data/dialogue.

As shown in the second command, commands don't need to have any tags.

In the third command, by including {} in the command name, you can specify a location to put a number, where a random number [2, 50] will be placed.

An example action:
```
(in data/objectives/action)
GOTO;KILL {};RET;

(resulting output)
1. Go to Hogwarts
2. Return here
3. Kill 14 goblins
```
