#include "shakva/driver.h"

int main(int argc, char **argv) {
    shk_driver_t d = shk_driver_new();
    int res = shk_driver_run(d, argc, argv);
    shk_driver_free(d);

    return res;
}
