#include <cstdio>
#include <fstream>
#include <iostream>

using std::ifstream;

using std::string;

string get_os() {
    string os;

    string line;
    ifstream file("/etc/os-release");
    while (getline(file, line)) {
        // TODO filter name
    }

    return os;
}


int main() {
    std::cout << get_os() << "\n";
}
