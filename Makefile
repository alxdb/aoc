all: compile_commands.json

build:
	cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -S . -B ./build

compile_commands.json: build
	ln -s build/compile_commands.json .
