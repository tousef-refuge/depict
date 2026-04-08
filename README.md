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
awesome

The reason I made this is because I made a prototype of this purely in python
which works almost the exact same way, just way jankier and needs a bit too
much typing, but with the power of spamming RustRover autocorrect, 2 hours of
sleep and chatgpt's useless code anything is possible

## Setup

- Make sure you have cargo, python and pip installed first
- Open the terminal and do ```cd dir``` where dir is any directory of your
  choice. Preferably somewhere kinda hidden though since once you're done with
  the setup you won't see this folder ever again I think lmao
- After that run the following set of commands:

**Windows:**
```commandline
py -m venv venv
venv/Scripts/activate
pip install -r requirements.txt
cargo install --path . --force
```

**MacOS/Linux:**
```commandline
py -m venv venv
source venv/bin/activate
pip install -r requirements.txt
cargo install --path . --force
```

- And that's the setup done. Now you can run ```depict``` from anywhere on your
  terminal. Instructions on how to use it are hopefully clear enough once you
  run ```depict --help```
- Additionally, if you use Windows you can just download a release directly on this
  github page. I don't feel like figuring out how to run it though, it's kind of a
  long story and I only wanna add releases to implement auto-updating later lmao
- If for SOME REASON you ever feel dissatisfied with depict despite how AWESOME
  it is and want to get rid of it forever, you can do ```cargo uninstall depict```
  at any time. This still leaves the actual files up for deletion though.

## Changelog

- (1.0.0) Initial release. Has the ability to trim, resize, rescale and flip images.
  All these functions can also recursively process images in a directory and all its
  sub-directories too