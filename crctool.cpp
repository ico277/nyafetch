#include <iostream> 
#include "./src/crc.hpp"

int main(int argc, char** argv) {
    for(int i = 1; i < argc; i++) {
        char* arg = argv[i];
        std::cout << "strcrc32('" << arg << "') -> " << strcrc32(arg) << "\n";  
    }
}