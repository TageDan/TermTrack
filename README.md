# TermTrack
A terminal-rendered 3D platforming/maze game with focus on speedrunning and custom level creation

https://github.com/user-attachments/assets/27e9e3ad-81ff-4e92-977d-c8ae899da69b

## Requirements
You will need a terminal to play this game. But all teminals are not created the same. For the moment we recommend that you use [windows-terminal](https://www.microsoft.com/store/productId/9N0DX20HK701?ocid=pdpshare) availible in the microsoft store (further testing will be done in the future)

## Install
### Windows
Install the zip-folder from the releases section or using the link: [https://tagedan.github.io/TermTrack/TERMTRACK_WINDOWS.zip](https://termtrack.github.io/TermTrack/TERMTRACK_WINDOWS.zip)
Extract it into a folder of your choice.
### Linux
Install the zip-folder from the releases section or using the link: [https://tagedan.github.io/TermTrack/TERMTRACK_LINUX.zip](https://termtrack.github.io/TermTrack/TERMTRACK_LINUX.zip)
Extract it into a folder of your choice.

## Run
In the extracted "TermTrack" folder run:
```bash
example/TermTrack> termtrack level_pack_0
```
where ```level_pack_0``` can be substituted for the name of the folder containing the levels you want to play.

### From source
Unfortunaly, due to the need of a secret salt to validate the leaderboard you cannot build this project from source and expect it to work with the leaderboard. We are working on a seperate branch where the leaderboard will instead be local and therefore can be built from source.

## Level Layout/Creation
A level is represented by a textfile with the format level_name.txt (or any other file extention, everything up until the last '.' will be the level name)
To build a level you write characters that will represent the grid of the actual level. There are 8 grid-types at the moment, these are:

- 'S' (start grid)
- 'E' (end grid)
- 'X' (wall)
- 'x' (half-wall / stair)
- 'v' (spike)
- '.' (floor)
- ' ' (hole)
- 'e' (enemy / angry-pixel spawn)

There is also the floor seperator represented by a new row containing only *sep* after wich the next floor can be built.


Example_level.txt:
``` 
XXX
XSX
XvX
X.X
XxX
XXX
sep
XXXXXXX
X....EX
X.XXXXX
X.X
X X
XXX
```
This level will have two floors with the lower floor containing the start and the stair to the second floor as well as a spike between them and the upper floor containing the end.
To then play the lavel you need to put it into a folder next to termtrack.exe and then run:
```bash
TermTrack> termtrack <level_folder_name> 
```
replacing `<level_folder_name>` with the name of your folder.

## Known bugs
Lazy error handling of network-requests leads to game crashing when offline. 
Leaderboard validation is faulty. 
Terminal focus can be hard to regain when lost.
Music crackling on high load.

## Future plans
- [X] [Standalone Leaderboard](https://github.com/TermTrack/TrackLeaderboard)
- [ ] Enemy sound
- [ ] [Level Editor](https://github.com/TermTrack/TrackEditor)
- [ ] Discord Bot
- [ ] 3D-object file loader

## Support
If you have any questions, please look at the github-discussions tab to see if it has already been answered or start a new discussion.

## Contributing
Feel free to create a issue if you have a feature that you would like to see implemented. You are also free to fork the project and make a pr if you have made an improvement.

## Acknowledgements
- [abbfelarb](https://github.com/abbfelarb) - Owner
- [TageDan](https://github.com/TageDan) - Owner
- [GustavPetterssonBjorklund](https://github.com/GustavPetterssonBjorklund) - For suggesting security actions
- https://patorjk.com/software/taag/ - for generating the title-art
- The Rust programming language and The terminal :) - for making this possible

