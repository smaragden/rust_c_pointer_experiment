#include <stdlib.h>
#include <stddef.h>

#define GP_OK 0

#define C_MEM(MEM)         \
    do                     \
    {                      \
        if ((MEM) == NULL) \
        {                  \
            return 2;      \
        }                  \
    } while (GP_OK)

#define C_PARAMS(PARAMS) \
    do                   \
    {                    \
        if (!(PARAMS))   \
        {                \
            return 3;    \
        }                \
    } while (GP_OK)

struct _CameraFile
{
    char mime_type[64];
};

typedef struct _CameraFile CameraFile;

int gp_file_new(CameraFile **file);
int gp_file_get_mime_type  (CameraFile *file, const char **mime_type);
int gp_file_free(CameraFile *file);
