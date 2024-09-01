# tkd-scoreboard

[![rust](https://img.shields.io/badge/rust-1.73.0-dea584)](https://releases.rs/docs/1.73.0/)
[![fltk-rs](https://img.shields.io/badge/fltk-%5E1.2-6f71b5)](https://crates.io/crates/fltk)
[![fltk-rs](https://img.shields.io/badge/chrono-0.2.16-9978a3)](https://crates.io/crates/chrono)
[![fltk-rs](https://img.shields.io/badge/directories-4.0.1-b57d96)](https://crates.io/crates/directories)
[![fltk-rs](https://img.shields.io/badge/cacache-11.3.0-de8383)](https://crates.io/crates/cacache)

This project aims to provide a simple taekwondo scoreboard program for the best of 3 rounds kyorugi contest system, with the World Taekwondo competition rules in force as of the 1st of September of 2022.

Executables for Windows and GNU/Linux systems can be downloaded from the [releases section of this repository](https://github.com/irzinfante/tkd-scoreboard/releases).

## Linux desktop entry

To add this program as an app to be launched from apps menu in create the file `tkd-scoreboard.desktop` in `~/.local/share/applications` with the following content:

```
[Desktop Entry]
Type=Application
Name=TKD Scoreboard
Comment=Scoreboard program for taekwondo competition
Icon=tkd-scoreboard
Exec=tkd-scoreboard
Terminal=false
Categories=Game;Utility;
```

Put the Unix-like binary from releases downloads in `~/.local/bin` (remember to modify its permissions to allow execution) and the [icon file](assets/tkd-scoreboard.svg) in `~/.icons` to make them available for the desktop entry file.

## Lifecycle flowchart

![lifecycle.png](lifecycle.png)

## Screenshots

| ![new-execution.png](screenshots/display_Settings.png) |
|:--:|
| *Before each contest, contest settings can be configured. Data from already finalized contest can be exported and deleted..* |

| <table><tbody><tr><td style="width:400px">![display_CallContestants.png](screenshots/display_CallContestants.png)</td><td style="width:400px">![display_Round.png](screenshots/display_Round.png)</td></tr><tr><td style="width:400px">![display_Timeout.png](screenshots/display_Timeout.png)</td><td style="width:400px">![display_MedicalTimeout.png](screenshots/display_MedicalTimeout.png)</td></tr><tr><td style="width:400px">![display_KeumanCondition.png](screenshots/display_KeumanCondition.png)</td><td style="width:400px">![display_EndContest.png](screenshots/display_EndContest.png)</td></tr><tr><td style="width:400px">![display_RestFirstPart.png](screenshots/display_RestFirstPart.png)</td><td style="width:400px">![display_RestSecondPart.png](screenshots/display_RestSecondPart.png)</td></tr><tr><td style="width:400px">![display_SuperiorityDecision.png](screenshots/display_SuperiorityDecision.png)</td><td style="width:400px">![display_ContestWinner.png](screenshots/display_ContestWinner.png)</td></tr></tbody></table> |
|:--:|
| *Different stages of the contest from start to the end of the match.* |

| <table><tbody><tr><td style="width:400px">![screen_CallContestants.png](screenshots/screen_CallContestants.png)</td><td style="width:400px">![screen_Round.png](screenshots/screen_Round.png)</td></tr><tr><td style="width:400px">![screen_Timeout.png](screenshots/screen_Timeout.png)</td><td style="width:400px">![screen_MedicalTimeout.png](screenshots/screen_MedicalTimeout.png)</td></tr><tr><td style="width:400px">![screen_KeumanCondition.png](screenshots/screen_KeumanCondition.png)</td><td style="width:400px">![screen_EndContest.png](screenshots/screen_EndContest.png)</td></tr><tr><td style="width:400px">![screen_RestFirstPart.png](screenshots/screen_RestFirstPart.png)</td><td style="width:400px">![screen_RestSecondPart.png](screenshots/screen_RestSecondPart.png)</td></tr><tr><td style="width:400px">![screen_SuperiorityDecision.png](screenshots/screen_SuperiorityDecision.png)</td><td style="width:400px">![screen_ContestWinner.png](screenshots/display_ContestWinner.png)</td></tr></tbody></table> |
|:--:|
| *Same as previous but from the screen view.* |

## License

Copyright (C) 2022-2024 Iker Ruiz de Infante Gonzalez iker@irzinfante.eu

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

[LICENSE](LICENSE) contains a copy of the full GPLv3 licensing conditions.
