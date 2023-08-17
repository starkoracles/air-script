# Prerequistes
## Ocaml 4.13
You will need Ocaml 4.13. If you are not an Ocaml programmer and don't know how to manage it,
I stronly recommend building and installing directly from the GitHub repository and following
the vendors instructions. Felix only uses the base install, it does not require any extra
Ocaml tools, nor does it requires any ppx or third party packages.
It is ONLY used to build the Felix compiler. It it not required to run Felix.

## Python 3.8
Python is ONLY used to build the bootstrap version of Felix.
Felix then rebuilds itself from the bootstrap without needing Python.
Upgrades to Felix can ususally be performed without Python.
But you should have it anyhow!
The build uses `popen` which has been changed in Python versions and some other
things which mean it will not build with too early version and probably not
the current version either. But you can try.

## C++17
On Linux, you can use g++ or clang++. 
On Mac it is strongly recommended to use the latest Xcode version of clang++.

## SDL2
This is optional. If you install these components:
```
sdl2
sdl2_ttf
sdl2_image
```
the Felix GUI may work. However it is not required for STARLK testing stuff.
If the GUI tests don't work, you will need to copy these files:
```
cp build/release/host/config/sdl2.fpc ~/.felix/config
cp build/release/host/config/sdl2_ttf.fpc ~/.felix/config
cp build/release/host/config/sdl2_image.fpc ~/.felix/config
```
and edit the copies to reflect the actual location of the components.
If you edit the originals, a full rebuild of Felix will clobber your
changes, so edit the copies in the override directory instead.

# Clone
cd to directory `<parent>`
Clone the Felix repo and cd into it and follow the steps
```
https://github.com/felix-lang/felix.git)https://github.com/felix-lang/felix.git
cd felix
mkdir ~/.felix
mkdir ~/.felix/config
echo "FLX_INSTALL_DIR: $PWD" > ~/.felix/config/felix.fpc
```
Now you need to add the location of the program binaries to your PATH in your login shell setup file.
And you need to add the location of the library binaries to your LD_LIBRARY_PATH (on Linux) 
or DYLD_LIBRARY_PATH on MacOS. These locations are
```
$PWD/build/release/host/bin
```
and
```
$PWD/build/release/host/lib/rtl
```
respectively.

# Building
Just type
```
make
```

The build process used a build tool written in Python to build a bootstrap version of Felix.
Then, Felix is rebuilt, this time using the bootstrap version of Felix to build the production version.
Then, a very long time is spent running regression tests.

# Done
Now it's all done. Although Felix can be installed I recommend
DO NOT INSTALL FELIX.

Installation uses Unix standard locations
```
/usr/local/felix/felix-version
```
for the system and also COPIES the binaries to
```
/usr/local/bin
```
and the run time libraries to
```
/usr/local/lib
```
The problem with this is it makes it hard to remove Felix,
and very hard to reliably rebuild it (because you are never sure
which version or what you're using). So just
DO NOT INSTALL FELIX.

# Check
```
flx hello.flx
```
should run from the repo. Now
```
cd ..
flx felix/hello.flx
```
should also work. 




