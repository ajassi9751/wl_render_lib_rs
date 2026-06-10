# wl_render_lib_rs
This is a rust library for rendering things as a wayland client\
It contains utils in the utils module which contains unit types and other helpful types like NotNull\
The main way to interact with the library is with the image struct (in the render module)\
The image struct contains an in memory copy of the pixel representatin of an image, the width of the image, the height of the image, and a pointer to write data to (you get the pointer from the mmap call)\
The image struct should be created once per image that you want to display, this is because you can mutate the image through an ImageRequest, It also could create race conditions if done otherwise\
Using the image request, you can do many things to an image such as rotation and translation, then apply the request to the image\
To create an image, you need to use a backend, the only one that exists right now is the qoi backend which can parse a qoi image
