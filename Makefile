CXX = c++
CXXFILES = $(wildcard src/*.cpp)
override CXXFLAGS += -std=c++20 -Wall
override LDFLAGS += -ltomlplusplus -lpci
PREFIX = /usr/local
EXECUTABLE = nyafetch

.PHONY: build install uninstall clean run debug 

build:
	$(CXX) $(CXXFILES) $(LDFLAGS) $(CXXFLAGS) -o $(EXECUTABLE).out

install: ./$(EXECUTABLE).out
	cp ./$(EXECUTABLE).out $(PREFIX)/bin/$(EXECUTABLE)

uninstall: $(PREFIX)/bin/$(EXECUTABLE)
	rm $(PREFIX)/bin/$(EXECUTABLE) || true

clean:
	rm ./*.out 2> /dev/null || true
	rm ./vgcore* 2> /dev/null || true

run: build
	./$(EXECUTABLE).out

debug: clean
	$(CXX) -DDEBUG $(CXXFILES) -o $(EXECUTABLE)_debug.out $(LDFLAGS) $(CXXFLAGS) -g
#	./$(EXECUTABLE)_debug.out $(RUNARGS)

