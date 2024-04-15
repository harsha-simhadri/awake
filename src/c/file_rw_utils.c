#include <stdio.h>

typedef unsigned int u32;
typedef unsigned char u8;

typedef struct Metadata
{
    u32 npts_u32;
    u32 ndims_u32;
} Metadata;

int open_fp_to_read(char *filename, FILE **fp)
{
    if (*fp != NULL)
    {
        printf("Error: fp is not NULL\n");
        return -1;
    }
    *fp = fopen(filename, "rb");
    if (*fp == NULL)
    {
        printf("Error: cannot open file %s\n", filename);
        return -1;
    }
    else
    {
        printf("File %s opened\n", filename);
    }
    return 0;
}

int open_fp_to_rw(char *filename, FILE **fp)
{
    if (*fp != NULL)
    {
        printf("Error: fp is not NULL\n");
        return -1;
    }
    *fp = fopen(filename, "r+b");
    if (*fp == NULL)
    {
        printf("Error: cannot open file %s\n", filename);
        return -1;
    }
    else
    {
        printf("File %s opened\n", filename);
    }
    return 0;
}

int drop_fp(FILE *fp)
{
    return fclose(fp);
}

int read_metadata(FILE *fp, Metadata const *metadata)
{
    if (fp == NULL)
    {
        printf("Error: fp is NULL\n");
        return -1;
    }
    fseek(fp, 0, SEEK_SET);
    fread((void *)&metadata->npts_u32, sizeof(u32), 1, fp);
    fread((void *)&metadata->ndims_u32, sizeof(u32), 1, fp);
    if (ferror(fp))
    {
        printf("Error: cannot read metadata from fp\n");
        return -1;
    }
    return 0;
}

int get_vector_f32(FILE *fp, const Metadata const *metadata, size_t id, float *vec)
{
    if (id >= metadata->npts_u32)
    {
        printf("Error: id out of range\n");
        return -1;
    }
    fseek(fp, sizeof(u32) * 2 + sizeof(float) * id * metadata->ndims_u32, SEEK_SET);
    fread(vec, sizeof(float), metadata->ndims_u32, fp);
    if (ferror(fp))
    {
        printf("Error: cannot read vector from fp\n");
        return -1;
    }
    return 0;
}

int get_vector_u8(FILE *fp, const Metadata const *metadata, size_t id, u8 *vec)
{
    if (id >= metadata->npts_u32)
    {
        printf("Error: id out of range\n");
        return -1;
    }
    fseek(fp, sizeof(u32) * 2 + sizeof(u8) * id * metadata->ndims_u32, SEEK_SET);
    fread(vec, sizeof(u8), metadata->ndims_u32, fp);
    if (ferror(fp))
    {
        printf("Error: cannot read vector from fp\n");
        return -1;
    }
    return 0;
}

int set_vector_f32(FILE *fp, const Metadata const *metadata, size_t id, float *vec)
{
    if (id >= metadata->npts_u32)
    {
        printf("Error: id out of range\n");
        return -2;
    }
    fseek(fp, sizeof(u32) * 2 + sizeof(float) * id * metadata->ndims_u32, SEEK_SET);
    fwrite((void*)vec, sizeof(float), metadata->ndims_u32, fp);
    if (ferror(fp))
    {
        printf("Error: cannot write vector to fp\n");
        return -1;
    }
    return 0;
}

int set_vector_u8(FILE *fp, const Metadata const *metadata, size_t id, u8 *vec)
{
    if (id >= metadata->npts_u32)
    {
        printf("Error: id out of range\n");
        return -2;
    }
    fseek(fp, sizeof(u32) * 2 + sizeof(u8) * id * metadata->ndims_u32, SEEK_SET);
    fwrite((void*)vec, sizeof(u8), metadata->ndims_u32, fp);
    if (ferror(fp))
    {
        printf("Error: cannot write vector to fp\n");
        return -1;
    }
    return 0;
}

int create_empty_f32bin_of_size(char *filename, const Metadata const *metadata)
{
    FILE *fp = fopen(filename, "wb");
    if (fp == NULL)
    {
        printf("Error: cannot create file %s\n", filename);
        return -1;
    }
    fwrite((void *)&metadata->npts_u32, sizeof(u32), 1, fp);
    fwrite((void *)&metadata->ndims_u32, sizeof(u32), 1, fp);
    float zero = 0.0;
    for (u32 i = 0; i < metadata->npts_u32; i++)
    {
        for (u32 j = 0; j < metadata->ndims_u32; j++)
        {
            fwrite((void *)&zero, sizeof(float), 1, fp);
        }
    }
    fclose(fp);
    return 0;
}

int create_empty_u8bin_of_size(char *filename, const Metadata const *metadata)
{
    FILE *fp = fopen(filename, "wb");
    if (fp == NULL)
    {
        printf("Error: cannot create file %s\n", filename);
        return -1;
    }
    fwrite((void *)&metadata->npts_u32, sizeof(u32), 1, fp);
    fwrite((void *)&metadata->ndims_u32, sizeof(u32), 1, fp);
    u8 zero = 0;
    for (u32 i = 0; i < metadata->npts_u32; i++)
    {
        for (u32 j = 0; j < metadata->ndims_u32; j++)
        {
            fwrite((void *)&zero, sizeof(u8), 1, fp);
        }
    }

    fclose(fp);
    return 0;
}

void print_vector_f32(float *vec, u32 ndims)
{
    for (u32 i = 0; i < ndims; i++)
    {
        printf("%.2f ", vec[i]);
    }
    printf("\n");
}