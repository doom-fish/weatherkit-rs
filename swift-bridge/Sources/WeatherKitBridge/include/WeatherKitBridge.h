#ifndef WEATHERKIT_BRIDGE_H
#define WEATHERKIT_BRIDGE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void wk_string_free(char *s);
int32_t wk_weather_for(double latitude, double longitude, char **out_json, char **out_error);

#ifdef __cplusplus
}
#endif

#endif
