# topdown_game
Currently, this game is an animated sprite that can move within a set of boundaries on a map dynamically generated based
on Tiled configuration.

# UPDATE 12/04/23
On the side, I have been learning how to use OpenGL APIs directly with C/C++ and ultimately rewrote this project from
Rust to C/C++ over the course of a few months. Today is the day all the files got dropped in the repository

# Building the Project

In order to build and run the project you will need to download these dependencies onto your local computer:

- [STB](https://github.com/nothings/stb) an image parsing library
- [GLEW](https://glew.sourceforge.net/) you will have to install this as an application on your computer in order for it to be usable
- [OpenGL](https://www.opengl.org/) this comes with most systems as it is a long existing legacy graphics API
- [glm](https://github.com/g-truc/glm) matrix and vector math
- [glfw3](https://github.com/glfw/glfw) GLFW helper library
- [nlohmann_json](https://github.com/nlohmann/json) JSON parsing library

## Notes for STB, glm, glfw3, and nlohmann_json

You should be able to create a lib folder on you computer ex: /usr/path/to/programming/folders/lib

Then you download your projects into that folder. Usually in the terminal I do `cd /usr/path/to/programming/folders/lib`
This changes the folder I am in to the "lib" folder. Then while in the lib folder I clone the projects I need by doing `git clone`
like so `git clone https://github.com/g-truc/glm`

In the end your lib folder should have these new folders:
- /usr/path/to/programming/folders/lib/glm
- /usr/path/to/programming/folders/lib/json
- /usr/path/to/programming/folders/lib/stb
- /usr/path/to/programming/folders/lib/glfw

Then you have to build each project using CMake and then link this project to the CMake config file by adding the CMake config
file to the prefix path. Here is an example of flags you have to pass into CMake to make this project work:

```
-DSTB_LIBRARY=/usr/path/to/programming/folders/lib/stb
-DCMAKE_PREFIX_PATH=/usr/path/to/programming/folders/lib/json/cmake-build;/usr/path/to/programming/folders/lib/glm/cmake/glm
```

For clarification here are the files CMake will look at now to link glm to this project:
- /usr/path/to/programming/folders/lib/glm/cmake/glm/**glmConfig-version.cmake**
- /usr/path/to/programming/folders/lib/glm/cmake/glm/**glmConfig.cmake**

*Sidenote: glfw3 was installed on my computer already, so CMake just picked it up from the installed apps folder*

## Notes on GLEW installation:
If you use a package manager like [brew](https://brew.sh/), you should be able to do something like `brew install glew` 
and then **brew** will install the package for you in a directory that CMake is likely to read from to build your project.

On Windows I suggest taking a look at this [page](https://glew.sourceforge.net/install.html) for installing GLEW. You
literally have to drop a glew DLL into your System32 folder in order for it to work.

# credit
Credit for assets: [Cup Nooble](https://cupnooble.itch.io/) 