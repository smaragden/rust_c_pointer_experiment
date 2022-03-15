#include <stdio.h>
#include <string.h>
#include <gp.h>

int gp_file_new(CameraFile **file)
{
    printf("[DEBUG] gp_file_new\n");
    C_PARAMS(file);

    C_MEM(*file = calloc(1, sizeof(CameraFile)));
    printf("[DEBUG] gp_file_new | file address: 0x%x\n", *file);

    strcpy((*file)->mime_type, "unknown/unknown");
    return (GP_OK);
}

int gp_file_get_mime_type(CameraFile *file, const char **mime_type)
{
    printf("[DEBUG] gp_file_get_mime_type\n");
    C_PARAMS(file && mime_type);

    *mime_type = file->mime_type;
    printf("[DEBUG] gp_file_get_mime_type | mime_type address: 0x%x\n", *mime_type);

    return (GP_OK);
}

int gp_file_free(CameraFile *file)
{
    printf("[DEBUG] gp_file_free\n");
    C_PARAMS(file);

    free(file);
    return (0);
}
