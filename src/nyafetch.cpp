#include <fstream>
#include <iostream>

#include "nyafetch.hpp"

using std::ifstream;
using std::string;

string get_os() {
    string line;

    ifstream file("/etc/os-release");
    while (getline(file, line)) {
        // check if line starts with 'ID='
        if (line.find("ID=") == 0) {
            // cut out 'ID='
            line.replace(0, 3, "");
            break;
        }
    }

    return line;
}

int main() {
    // what to print
    //InfoUnion_t info_union;
    //info_union.value = 0xFFFFFFFFFFFFFFFF;
    for (int i = 0; i < sizeof(info_union.fields); i++) {
        printf("%d\n", info_union.fields[i]);
    }
    // TODO add argument and config parsing  
    if (info_union.fields[0]) {
        std::cout << get_os() << "\n";
    }
}
