<div style="text-align: center;">
    <img src="./logo.svg" alt=" " width="128"><br/>
  <sub><em>depict, a CLI tool for fast image editing</em></sub>
</div>

## Overview

Okay this is my first actually somewhat serious project for once. The core
idea of it is to make mundane tasks like removing transparent filler off of
an image, doubling the size of several images etc. super easy and straigtforward
to do. Just open the terminal and do a ```depict trim C:/Pictures``` and all
your problems are cured and your college debt is paid off and everything is
awesome. This is really helpful for trimming multiple transparent photos
perfectly, adjusting game assets, AI data augmentation, mass-flipping all your 
art to easily find anatomy mistakes etc.

The reason I made this is because I made a prototype of this for my game purely 
in python which works almost the exact same way, just way jankier and needs a bit 
too much typing. I ended up forgetting about the game but I really like the idea
of the python CLI so with the power of spamming RustRover autocorrect, 2 hours of
sleep and chatgpt's useless code anything is possible

## Setup

- There's actually two ways you can do this. I recommend the first one but if
  that doesn't end up working then you have the second more lengthy option

**Method 1: By downloading github releases (only works for Windows and Linux):**

- Make sure you have python and pip installed first
- Go to the releases section on this page to the right
- Download a .zip with the version you want and the right OS
- Extract it somewhere, like your desktop
- Copy the address of the zip you just extracted and put it in environmental path
  variables (for reference, at least on Windows the path should look something like
  ```C:\Users\tousef-refuge\depict-v1.1.0-x86_64-windows\```). To learn how to
  actually do this search up a video or something cause apparently it differs 
  from OS to OS
- If you downloaded a release starting from 1.2.0 onwards, congrats the setup is over
  and you can run ```depict``` from anywhere on the terminal! If you downloaded an 
  older release though you might still want to continue from here. Make sure you
  **DO NOT** delete the release.marker file in the directory otherwise ```depict update``` won't
  work anymore. If ```depict update``` ever goes wrong the files from the update you
  migrated off of will still be there with a .old extension at the end. These only get
  deleted on the next successful run of the new update (which obviously isn't happening
  if the update went badly in the first place lmao). Removing the .old extension and
  running them as usual will still work. Keep in mind from v1.2.0 onwards the program 
  deletes **ALL** .old files inside this directory with every successful command run
- Open the terminal and do ```cd dir``` where dir is the directory that you extracted
  to just now
- After that run the following set of commands:

**Windows:**
```commandline
cd dir
py -m venv venv
venv/Scripts/activate
pip install -r requirements.txt
```

**Linux:**
```commandline
cd dir
py -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

- Keep in mind that the venv folder HAS to be called venv (not .venv) otherwise 
  the program will not work
- And that's the setup done. Now you can run ```depict``` from anywhere on your
  terminal. Instructions on how to use it are hopefully clear enough once you
  run ```depict --help```
- If this doesn't work somehow or if you don't have the right OS then you can 
  go to the next method

**Method 2: By manually cloning:**

- Make sure you have cargo, python and pip installed first. Also make sure you
  have some storage lying around since this method could take up some space
  (it takes up like 2gb as of v1.1.0)
- Open the terminal and do ```cd dir``` where dir is any directory of your
  choice. Preferably somewhere kinda hidden though since once you're done with
  the setup you won't see this folder ever again I think lmao
- Run the set of commands in the previous method exactly as they are (for macOS
  use the Linux way), but add ```cargo install --path . --force``` at the end
- You can get rid of the updater/ directory. It doesn't do anything if you
  git cloned the project anyway lmao
- If for SOME REASON you ever feel dissatisfied with depict despite how AWESOME
  it is and want to get rid of it forever, deleting the project isn't enough.
  You also need to do ```cargo uninstall depict``` to remove the file for good

## Changelog

- (1.0.0) Initial release. Has the ability to trim, resize, rescale and flip images.
  All these functions can also recursively process images in a directory and all its
  sub-directories too
- (1.1.0) Added update check and speed optimizations. Releases now support linux. Sorry
  macOS guys I couldn't figure it out yet and I also don't trust release.yml
- (1.2.0) Added two new commands: depict update to automatically upgrade the CLI whenever
  necessary (only works if you have it downloaded as a github release) and depict alpha
  to adjust the opacity of an image
- (1.3.0) Added two new flags for every image command: --ignore to skip processing specified
  files and --only to process only specified files. These flags also follow globbing patterns
  (for example ```depict trim C:/ --only **/test_*.png``` will only target files that have
  the format test_\<name\>.png). Added two new image commands: depict invert and depict
  grayscale. Take a wild guess at what these do