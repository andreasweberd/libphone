#ifndef PHONE_VERSION_H
#define PHONE_VERSION_H

#include "phone_export.h"
#include <stddef.h> //NOLINT deprecated-headers

#ifdef __cplusplus
extern "C"
{
#endif

PHONE_EXPORT unsigned phone_version_major(void);
PHONE_EXPORT unsigned phone_version_minor(void);
PHONE_EXPORT unsigned phone_version_patch(void);
PHONE_EXPORT unsigned phone_version_tweak(void);

PHONE_EXPORT void phone_git_hash(char *out, size_t size);
PHONE_EXPORT void phone_git_description(char *out, size_t size);

#ifdef __cplusplus
}
#endif

#endif //PHONE_VERSION_H
