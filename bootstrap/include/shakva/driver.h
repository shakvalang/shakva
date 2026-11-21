#pragma once

typedef struct shk_driver_impl_t *shk_driver_t;

shk_driver_t shk_driver_new(void);
void shk_driver_free(shk_driver_t);
int shk_driver_run(shk_driver_t, int, char **);
