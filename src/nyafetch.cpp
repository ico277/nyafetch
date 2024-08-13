#include <cstddef>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <array>
#include <sstream>
#include <string>
#include <tuple>
#include <csignal>

#include "nyafetch.hpp"
#include "crc.hpp"

extern "C" {
#include <pci/pci.h>
}

// index 0: ID
// index 1: NAME
std::array<std::string, 2> get_os() {
    std::array<std::string, 2> os = {"", ""};

    std::string line;
    std::ifstream file("/etc/os-release");
    while (getline(file, line)) {
        // if array is filled -> exit
        if (!os[0].empty() && !os[1].empty()) {
            break;
        }
        // check if line starts with 'ID='
        if (line.find("ID=") == 0) {
            // remove 'ID='
            replace_all(line, "ID=", "");
            // remove '"'
            replace_all(line, "\"", "");
            os[0] = line;
        } else if (line.find("NAME=") == 0) {
            // remove 'NAME='
            replace_all(line, "NAME=", "");
            // remove '"'
            replace_all(line, "\"", "");
            os[1] = line;
        }
    }

    file.close();
    return os;
}

std::string get_kernel_version() {
    std::string kernel_version = "";

    std::string line;
    std::ifstream file("/proc/version");
    if (getline(file, line)) {
        // create a stream out of the line
        std::stringstream line_stream(line);
        // the version is the third word so read 3 times
        for (int i = 0; i < 3; ++i) {
            getline(line_stream, kernel_version, ' ');
        }
    }

    file.close();
    return kernel_version;
}

// index 0: uptime
// index 1: idle time
std::string get_uptime() {
    std::ostringstream uptime;

    // read uptime into doubles
    std::ifstream file("/proc/uptime");
    double uptime_s, idle_s;
    file >> uptime_s;
    file >> idle_s;

    // calculate uptime
    int uptime_days = uptime_s / (24 * 3600);
    uptime_s = fmod(uptime_s, 24 * 3600);
    int uptime_hours = uptime_s / 3600;
    uptime_s = fmod(uptime_s, 3600);
    int uptime_minutes = uptime_s / 60;

    // format uptime string
    if (uptime_days > 0) {
        uptime << uptime_days << " days";
        uptime << ", ";
    }
    if (uptime_hours > 0) {
        uptime << uptime_hours << " hours";
        uptime << ", ";
    }
    if (uptime_minutes > 0) {
        uptime << uptime_minutes << " minutes";
    }

    file.close();
    return uptime.str();
}

// 0: CPU_MODEL
// 1: CPU_FREQ
// 2: CPU_CORES
std::array<std::string, 3> get_cpuinfo() {
    std::array<std::string, 3> cpuinfo = {"", "", ""};
    int cores = 0;

    std::string line;
    std::ifstream file("/proc/cpuinfo");
    while (getline(file, line)) {
        if (line.find("model name", 0) == 0 && cpuinfo[0].empty()) {
            std::size_t colon_index = line.find(":") + 2;
            line.replace(0, colon_index, "");
            cpuinfo[0] = line;
        } else if (line.find("cpu MHz", 0) == 0 && cpuinfo[1].empty()) {
            std::size_t colon_index = line.find(":") + 2;
            line.replace(0, colon_index, "");
            cpuinfo[1] = line;
        } else if (line.find("processor", 0) == 0) {
            ++cores;
        }
    }
    cpuinfo[2] = std::to_string(cores);

    file.close();
    return cpuinfo;
}

std::vector<std::string> get_gpu_names() {
    std::vector<std::string> gpus;

    // parse PCI devices
    struct pci_access *pciaccess = pci_alloc();
    pci_init(pciaccess);
    pci_scan_bus(pciaccess);

    for (struct pci_dev *dev = pciaccess->devices; dev; dev = dev->next) {
        char gpu_name[512];
        char gpu_vendor[512];
        memset(gpu_name, '\0', 512);
        memset(gpu_vendor, '\0', 512);

        pci_fill_info(dev, PCI_FILL_IDENT | PCI_FILL_CLASS | PCI_FILL_LABEL);
        switch (dev->device_class)
        {
        case 0x0380:
        case 0x0301:
        case 0x0302:
        case 0x0300: {
            pci_lookup_name(pciaccess, gpu_name, 512,
                            PCI_LOOKUP_VENDOR, dev->vendor_id);
            pci_lookup_name(pciaccess, gpu_vendor, 512,
                            PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id);
            std::string gpu;
            gpu += std::string(gpu_vendor) + " " + std::string(gpu_name);
            gpus.push_back(gpu);
            break;
        }
        default:
            continue;
        }
    }
    pci_cleanup(pciaccess);
    return gpus;
}

// 0: MEM_TOTAL
// 1: MEM_AVAILABLE
// 2: MEM_USED
// 3: MEM_USED_PERCENT
std::array<std::string, 4> get_meminfo() {
    std::string memtotal_str = "";
    std::string memavail_str = ""; 

    std::string line;
    std::ifstream file("/proc/meminfo");
    while (getline(file, line)) {
        // if both are filled -> exit
        if (!memtotal_str.empty() && !memavail_str.empty()) {
            break;
        } else if (line.find("MemTotal:") == 0) {
            std::size_t colon_index = line.find(":") + 1;
            line.replace(0, colon_index, "");
            line = left_trim(line);
            memtotal_str = line;
            getline(std::istringstream(line), memtotal_str, ' ');
        } else if (line.find("MemAvailable:") == 0) {
            std::size_t colon_index = line.find(":") + 1;
            line.replace(0, colon_index, "");
            line = left_trim(line);
            getline(std::istringstream(line), memavail_str, ' ');
        }
    }

    std::size_t memtotal = std::stoull(memtotal_str) / 1024; // convert form kB to mB
    std::size_t memavail = std::stoull(memavail_str) / 1024; // convert form kB to mB
    std::size_t memused = memtotal - memavail;
    std::size_t memusedpercent = double(memused) / double(memtotal) * 100;

    return {
        std::to_string(memtotal) + "MiB", 
        std::to_string(memavail) + "MiB",
        std::to_string(memused) + "MiB",
        std::to_string(memusedpercent) + "%"
    };
}


// 0 art
// 1 height
// 2 width
std::tuple<std::string, size_t, size_t> get_distro_art(uint32_t crchash) {
    std::string str;
    size_t height, width;

    switch (crchash) {
        case 3646356822: // 'arch'
            str = "      /\\      \n"
                  "    ^/  \\^    \n"
                  "    /\\   \\  \n"
                  "   / ^ w ^\\   \n"
                  "  /   __   \\  \n"
                  " /   |  |  -\\ \n"
                  "/_-''    ''-_\\\n";
            height = 7;
            width = 14;
            break;
        case 3699236742: // 'artix'
            str = "      /\\      \n"
                  "    ^/:3\\^    \n"
                  "    / -_ \\    \n"
                  "   /    -_\\   \n"
                  "  /    _-  \\  \n"
                  " /  _-  -_  \\ \n"
                  "/_-        -_\\\n";
            height = 7;
            width = 14;
            break;
        case 2085247189: // 'linuxlite'
            str = "   /\\   \n"
                  "  /  \\  \n"
                  " / / /  \n"
                  "> w <   \n"
                  " \\ \\ \\ \\\n"
                  "  \\_\\_\\\n"
                  "     \\\n";
            height = 7;
            width = 8;
            break;
        case 3448321946: // 'ubuntu'
            str = "         _ \n"
                  "     ---(_)\n"
                  " _/  ---  \\\n"
                  "(_) |UwU|  \n"
                  "  \\  --- _/\n"
                  "     ---(_)\n";
            height = 6;
            width = 11;
            break;
        case 4081749186: // 'gentoo'
            str = " _-----_   \n"
                  "(       \\  \n"
                  "\\  0w0   \\ \n"
                  " \\        )\n"
                  " /      _/ \n"
                  "(     _-   \n"
                  "\\____-    \n";
            height = 7;
            width = 11;
            break;
        case 3262278895: // 'debian'
            str = "    _____   \n"
                  "   /  ___ \\ \n"
                  "  |  / ^w^ |\n"
                  "  |  \\____/ \n"
                  "   -_       \n"
                  "     -._    \n";
            height = 6;
            width = 12; 
            break;
        case 3154936481: // 'endeavouros'
            str = "      /\\     \n"
                  "   ^//  \\\\^  \n"
                  "   //    \\ \\ \n"
                  " / / ^ w ^) )\n"
                  "/_/___-- __- \n"
                  " /____--     \n";
            height = 6;
            width = 13;
            break;
        default:         // unknown distro
            str = "    ^#####^    \n"
                  "    ##OwO##    \n"
                  "    #######    \n"
                  "  ###########  \n"
                  " ############# \n"
                  "###############\n" 
                  " ############# \n"
                  "  ###########  \n";
            height = 8;
            width = 15;
            break;
    }

    return std::make_tuple(str, height, width);
}

void exit_handler(int _signal) {
    // enable line wrap and reset colors
    ENABLE_LINE_WRAP() << RESET;
    exit(-1);
}

int main(int argc, char** argv) {
    // signal handling
    signal(SIGINT, exit_handler);

    // TODO add argument parsing

    nyafetch::Config config;
    std::string config_path = getenv("HOME");
    config_path += "/.config/nyafetch.conf";
    if (std::filesystem::exists(config_path)) {
        config = nyafetch::Config(config_path);
    } else {
        nyafetch::Config::WriteDefaultConfig(config_path);
        std::cerr << "No config file found! wrote default config file to " << config_path << "\n";
    }

    std::array<std::string, 2> os_name = get_os();
    std::tuple<std::string, size_t, size_t> distro_art_tuple = get_distro_art(strcrc32(os_name[0].c_str()));
    std::string art = std::get<0>(distro_art_tuple);
    size_t art_height = std::get<1>(distro_art_tuple);
    size_t art_width = std::get<2>(distro_art_tuple) + 1;
    
    // print everything
    std::cout << config.distro_art_color << art << RESET;
    MOVE_CUR_UP(art_height);

    std::vector<std::string> lines = {};
    for(auto& element : config.order) {
        std::string line;
        switch (strcrc32(element.c_str())) {
            case 2239976324: { // 'OS'
                line += config.value_color + "OS    " + RESET;
                line += config.seperator_color + config.seperator + RESET;
                line += config.value_color + config.os + RESET;
                replace_all(line, "%OS_ID%", os_name[0]);
                replace_all(line, "%OS_NAME%", os_name[1]);
                break;
            }
            case 485031054: {  // 'KERNEL'
                std::string kernel_version = get_kernel_version();
                line += config.value_color + "Kernel" + RESET;
                line += config.seperator_color + config.seperator + RESET;
                line += config.value_color + config.kernel + RESET;
                replace_all(line, "%KERNEL_VERSION%", kernel_version);
                break;
            }
            case 471202914: {  // 'UPTIME'
                std::string uptime = get_uptime();
                line += config.value_color + "Uptime" + RESET;
                line += config.seperator_color + config.seperator + RESET;
                line += config.value_color + config.uptime + RESET;
                replace_all(line, "%UPTIME%", uptime);
                break;
            }
            case 3546729398: { // 'CPU'
                std::array<std::string, 3> cpuinfo = get_cpuinfo();
                line += config.value_color + "CPU   " + RESET;
                line += config.seperator_color + config.seperator + RESET;
                line += config.value_color + config.cpu + RESET;
                replace_all(line, "%CPU%", cpuinfo[0]);
                replace_all(line, "%CPU_FREQ%", cpuinfo[1]);
                replace_all(line, "%CPU_CORES%", cpuinfo[2]);
                break;
            }
            case 3564069738: { // 'GPU'
                std::vector<std::string> gpus = get_gpu_names();
                size_t gpus_size = gpus.size();
                for (size_t i = 0; i < gpus_size; i++) {
                    std::string gpu_line;
                    gpu_line += config.value_color + "GPU   " + RESET;
                    gpu_line += config.seperator_color + config.seperator + RESET;
                    gpu_line += config.value_color + config.gpu + RESET;
                    std::string gpu = gpus[i];
                    replace_all(gpu_line, "%GPU%", gpu);
                    if (i == (gpus_size - 1))
                        line = gpu_line;
                    else
                        lines.push_back(gpu_line);
                }
                break;
            }
            case 2874626576: { // 'MEMORY'
                std::array<std::string, 4> meminfo = get_meminfo(); 
                line += config.value_color + "Memory" + RESET;
                line += config.seperator_color + config.seperator + RESET;
                line += config.value_color + config.memory + RESET;
                replace_all(line, "%MEM_TOTAL%", meminfo[0]);
                replace_all(line, "%MEM_AVAILABLE%", meminfo[1]);
                replace_all(line, "%MEM_USED%", meminfo[2]);
                replace_all(line, "%MEM_USED_PERCENT%", meminfo[3]);
                break;
            }
            default:
                line = "~" + element + "~";
                break;
        }
        lines.push_back(line);
    }
    // print
    DISABLE_LINE_WRAP();
    for (auto& line : lines) {
        MOVE_CUR_RIGHT(art_width) << line << "\n";
    }
    if (lines.size() < art_height) {
        for (size_t i = 0; i < (art_height - lines.size()); i++)
            std::cout << "\n";
    }
    // re-enable line-wrap
    ENABLE_LINE_WRAP();
}
