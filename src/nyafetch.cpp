#include <cstddef>
#include <cstdlib>
#include <cstring>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <array>
#include <sstream>
#include <string>

#include "nyafetch.hpp"

extern "C" {
#include "pci/pci.h"
}

// index 0: ID
// index 1: NAME
std::array<std::string, 2> get_os() {
    std::array<std::string, 2> arr = {"", ""};

    std::string line;
    std::ifstream file("/etc/os-release");
    while (getline(file, line)) {
        // if array is filled -> exit
        if (!arr[0].empty() && !arr[1].empty()) {
            break;
        }
        // check if line starts with 'ID='
        if (line.find("ID=") == 0) {
            // remove 'ID='
            replaceAll(line, "ID=", "");
            // remove '"'
            replaceAll(line, "\"", "");
            arr[0] = line;
        } else if (line.find("NAME=") == 0) {
            // remove 'NAME='
            replaceAll(line, "NAME=", "");
            // remove '"'
            replaceAll(line, "\"", "");
            arr[1] = line;
        }
    }

    file.close();
    return arr;
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
        case 0x0300:
            pci_lookup_name(pciaccess, gpu_name, 512,
                            PCI_LOOKUP_VENDOR, dev->vendor_id);
            pci_lookup_name(pciaccess, gpu_vendor, 512,
                            PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id);
            break;
        default:
            continue;
        }

        std::string gpu;
        gpu += std::string(gpu_vendor) + " " + std::string(gpu_name);
        gpus.push_back(gpu);
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
        // if array is filled -> exit
        if (!memtotal_str.empty() && !memavail_str.empty()) {
            break;
        } else if (line.find("MemTotal:") == 0) {
            std::size_t colon_index = line.find(":") + 1;
            line.replace(0, colon_index, "");
            line = ltrim(line);
            memtotal_str = line;
            getline(std::istringstream(line), memtotal_str, ' ');
        } else if (line.find("MemAvailable:") == 0) {
            std::size_t colon_index = line.find(":") + 1;
            line.replace(0, colon_index, "");
            line = ltrim(line);
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

int main(int argc, char** argv) {
    nyafetch::Config config;
    // TODO finish config file handling
    std::string config_path = getenv("HOME");
    config_path += "/.config/nyafetch.conf";
    if (std::filesystem::exists(config_path)) {
        config = nyafetch::Config(config_path);
    } else {
        nyafetch::Config::WriteDefaultConfig(config_path);
        std::cerr << "No config file found! wrote default config file to " << config_path << "\n";
    }

#ifdef DEBUG
    auto vec2str = [](const std::vector<std::string>& v) -> std::string {
        std::ostringstream oss;
        oss << "[";
        for (size_t i = 0; i < v.size(); ++i) {
            oss << "\"" << v[i] << "\"";
            if (i != v.size() - 1) {
                oss << ", ";
            }
        }
        oss << "]";
        return oss.str();
    };
    std::cout << "config.os='" << config.os << "'\n"
              << "config.kernel='" << config.kernel << "'\n"
              << "config.uptime='" << config.uptime << "'\n"
              << "config.cpu='" << config.cpu << "'\n"
              << "config.gpu='" << config.gpu << "'\n"
              << "config.memory='" << config.memory << "'\n"
              << "config.order='" << vec2str(config.order) << "'\n"
              << "config.uwuify='" << config.uwuify << "'\n"
              << "config.seperator='" << config.seperator << "'\n"
              << "config.key_color='" << config.key_color << "'\n"
              << "config.seperator_color='" << config.seperator_color << "'\n"
              << "config.value_color='" << config.value_color << "'\n"
              << "config.distro_art_color='" << config.distro_art_color << "'\n";
#endif
    
    // TODO add argument parsing

    // Print info
    std::array<std::string, 2> name = get_os();
    std::cout << "OS_ID : " << name[0] << "\n";
    std::cout << "OS_NAME : " << name[1] << "\n";
    std::string kernel_version = get_kernel_version();
    std::cout << "KERNEL_VERSION : " << kernel_version << "\n";
    std::string uptime = get_uptime();
    std::cout << "UPTIME : " << uptime << "\n";
    std::array<std::string, 3> cpuinfo = get_cpuinfo();
    std::cout << "CPU : " << cpuinfo[0] << "\n";
    std::cout << "CPU_FREQ : " << cpuinfo[1] << "\n";
    std::cout << "CPU_CORES : " << cpuinfo[2] << "\n";
    std::vector<std::string> gpus = get_gpu_names();
    for (const auto& gpu : gpus) {
        std::cout << "GPU : " << gpu << "\n";
    }
    std::array<std::string, 4> meminfo = get_meminfo(); 
    std::cout << "MEM_TOTAL : " << meminfo[0] << "\n";
    std::cout << "MEM_AVAILABLE : " << meminfo[1] << "\n";
    std::cout << "MEM_USED : " << meminfo[2] << "\n";
    std::cout << "MEM_USED_PERCENT : " << meminfo[3] << "\n";
}
