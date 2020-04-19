type sdl-config &> /dev/null || echo -e "SDL2 not found, consider:\nsudo apt-get install libsdl2-2.0\nand,\nsudo apt-get install libsdl2-mixer-2.0-0 libsdl2-image-2.0-0 libsdl2-ttf-2.0-0" || exit 1

echo -e "Every dependencies are already installed. You can launch the application."
