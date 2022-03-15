#include <stdio.h>
#include <gp.h>

int main(void){
    CameraFile* file;
    gp_file_new(&file);

    const char *mimetype = NULL;
	gp_file_get_mime_type (file, &mimetype);
    printf("mime: %s\n", mimetype);

    gp_file_free(file);

    return 0;
}
