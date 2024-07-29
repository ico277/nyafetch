#include <algorithm>
#include <vector>

#include <toml++/toml.hpp>

inline void replaceAll(std::string& str, const std::string& from, const std::string& to) {
    if(from.empty()) {
        return; // Avoid infinite loop when 'from' is an empty string
    }
    std::size_t start_pos = 0;
    while((start_pos = str.find(from, start_pos)) != std::string::npos) {
        str.replace(start_pos, from.length(), to);
        start_pos += to.length(); // Move past the replacement
    }
}

inline std::string ltrim(const std::string& s) {
    std::string result = s;
    result.erase(result.begin(), std::find_if(result.begin(), result.end(), [](unsigned char ch) {
        return !std::isspace(ch);
    }));
    return result;
}

namespace nyafetch {

    class Config {
    public:
        Config(std::string filepath) {
            // parse toml config file
            toml::table config_tb = toml::parse_file(filepath);
            
            // read [format] into a table
            if (auto format_tb_val = config_tb.get("format")) {
                auto format_tb = format_tb_val->as_table();
                // read every single variable 
                // and if it doesnt exist in the table
                // it uses the default value specified at variable creation
                if (auto os_val = format_tb->get_as<std::string>("os")) {
                    this->os = os_val->get();
                }
                if (auto kernel_val = format_tb->get_as<std::string>("kernel")) {
                    this->kernel = kernel_val->get();
                }
                if (auto uptime_val = format_tb->get_as<std::string>("uptime")) {
                    this->uptime = uptime_val->get();
                }
                if (auto cpu_val = format_tb->get_as<std::string>("cpu")) {
                    this->cpu = cpu_val->get();
                }
                if (auto gpu_val = format_tb->get_as<std::string>("gpu")) {
                    this->gpu = gpu_val->get();
                }
                if (auto memory_val = format_tb->get_as<std::string>("memory")) {
                    this->memory = memory_val->get();
                }
                // Read order array
                if (auto order_array = format_tb->get("order")) {
                    std::vector<std::string> order;
                    for (auto& val : *order_array->as_array()) {
                        order.push_back(val.as_string()->get());
                    }
                    this->order = order;
                }
                if (auto uwuify_val = format_tb->get_as<bool>("uwuify")) {
                    this->uwuify = uwuify_val->get();
                }
                if (auto seperator_val = format_tb->get_as<std::string>("seperator")) {
                    this->seperator = seperator_val->get();
                }
            }
 

            // read [appearance] to table
            if (auto appearance_tb_val = config_tb.get("appearance")) {
                auto appearance_tb = appearance_tb_val->as_table();
                // read every single variable 
                // and if it doesnt exist in the table
                // it uses the default value specified at variable creation
                if (auto key_color_val = appearance_tb->get_as<std::string>("key_color")) {
                    this->key_color = key_color_val->get();
                    replaceAll(this->key_color, "\\x1b", "\x1b");
                }
                if (auto seperator_color_val = appearance_tb->get_as<std::string>("seperator_color")) {
                    this->seperator_color = seperator_color_val->get();
                    replaceAll(this->seperator_color, "\\x1b", "\x1b");
                }
                if (auto value_color_val = appearance_tb->get_as<std::string>("value_color")) {
                    this->value_color = value_color_val->get();
                    replaceAll(this->value_color, "\\x1b", "\x1b");
                }
                if (auto distro_art_color_val = appearance_tb->get_as<std::string>("distro_art_color")) {
                    this->distro_art_color = distro_art_color_val->get();
                    replaceAll(this->distro_art_color, "\\x1b", "\x1b");
                }
            }
        }

        Config(){}

        // format
        std::string os = "%OS_NAME%";
        std::string kernel = "Linux %KERNEL_VERSION%";
        std::string uptime = "%UPTIME%";
        std::string cpu = "%CPU% (%CPU_CORES%) @ %CPU_FREQMHz";
        std::string gpu = "%GPU%";
        std::string memory = "%MEM_USED%/%MEM_TOTAL% (%MEM_USED_PERCENT%)";
        std::vector<std::string> order = {"OS", "KERNEL", "UPTIME", "CPU", "GPU", "MEMORY"};
        std::string seperator = " -> ";
        bool uwuify = true;
        // appearance
        std::string key_color = "\x1b[38;5;213m";
        std::string seperator_color = "\x1b[38;5;213m";
        std::string value_color = "\x1b[38;5;7m";
        std::string distro_art_color = "\x1b[38;5;213m";

        static Config WriteDefaultConfig(std::string filepath) {
            std::ofstream file(filepath);
            file << R"configfile([format]
os     = "%OS_NAME%"               # possible values: OS_NAME OS_ID
kernel = "Linux %KERNEL_VERSION%"  # possible values: KERNEL_VERSION 
uptime = "%UPTIME%"                # possible values: UPTIME
cpu    = "%CPU% (%CPU_CORES%) @ %CPU_FREQMHz"           # possible values: CPU CPU_CORES CPU_FREQ
gpu    = "%GPU%"                                      # possible values: GPU
memory = "%MEM_USED%/%MEM_TOTAL% (%MEM_USED_PERCENT%)" # possible values: MEM_USED MEM_TOTAL MEM_AVAILABLE MEM_USED_PERCENT
order  = ["OS", "KERNEL", "UPTIME", "CPU", "GPU", "MEMORY"]
uwuify = true
seperator = " -> "

[appearance]
# ANSI escape codes
key_color = "\\x1b[38;5;213m"
seperator_color = "\\x1b[38;5;7m"
value_color = "\\x1b[38;5;213m"
distro_art_color = "\\x1b[38;5;213m"
)configfile";
            file.close();
            return Config(filepath);          
        }
    };

}