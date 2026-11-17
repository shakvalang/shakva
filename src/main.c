#include "shakva/driver.h"

int main(int argc, char **argv) {
    shk_driver_parsed_args_t args;
    int parse_failed = shk_driver_parse_args(&args, argc, argv);
    if (parse_failed) {
        return parse_failed;
    }

    return shk_driver_run(&args);
}
