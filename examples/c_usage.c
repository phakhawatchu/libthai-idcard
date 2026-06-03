/**
 * c_usage.c — Example of using libthaiidcard from C.
 *
 * Two approaches are shown:
 *   1. Dynamic loading (dlopen/dlsym) — no linker flags needed at compile time
 *   2. Compile-time linking — link with -lthaiidcard
 *
 * Compile (dynamic):
 *   cc -o c_usage examples/c_usage.c -ldl
 *   ./c_usage
 *
 * Compile (static linking):
 *   cc -o c_usage examples/c_usage.c -Ltarget/debug -lthaiidcard \
 *      -lpcsclite -Wl,-rpath,target/debug
 *   ./c_usage
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dlfcn.h>

/* ---------------------------------------------------------------------------
 * Opaque handle — matches the Rust definition
 * -------------------------------------------------------------------------*/
typedef struct thaiid_card_data thaiid_card_data;

/* ---------------------------------------------------------------------------
 * Function pointer typedefs for the C API
 * -------------------------------------------------------------------------*/
typedef thaiid_card_data *(*read_fn)(const char *reader, int face, int nhso, int laser);
typedef void (*free_fn)(thaiid_card_data *data);
typedef const char *(*get_last_error_fn)(void);

typedef const char *(*get_str_fn)(const thaiid_card_data *data);

/* ---------------------------------------------------------------------------
 * Function pointers (populated by load_library)
 * -------------------------------------------------------------------------*/
static read_fn fn_read = NULL;
static free_fn fn_free = NULL;
static get_last_error_fn fn_get_last_error = NULL;

static get_str_fn fn_get_cid = NULL;
static get_str_fn fn_get_name_thai = NULL;
static get_str_fn fn_get_name_en = NULL;
static get_str_fn fn_get_dob = NULL;
static get_str_fn fn_get_gender = NULL;
static get_str_fn fn_get_card_issuer = NULL;
static get_str_fn fn_get_issue_date = NULL;
static get_str_fn fn_get_expire_date = NULL;
static get_str_fn fn_get_address = NULL;
static get_str_fn fn_get_face_image = NULL;
static get_str_fn fn_get_laser_id = NULL;
static get_str_fn fn_get_main_inscl = NULL;
static get_str_fn fn_get_sub_inscl = NULL;
static get_str_fn fn_get_main_hospital = NULL;
static get_str_fn fn_get_sub_hospital = NULL;
static get_str_fn fn_get_paid_type = NULL;
static get_str_fn fn_get_nhso_issue_date = NULL;
static get_str_fn fn_get_nhso_expire_date = NULL;
static get_str_fn fn_get_nhso_update_date = NULL;
static get_str_fn fn_get_change_hospital_amount = NULL;

/* ---------------------------------------------------------------------------
 * Load all symbols from the shared library
 * -------------------------------------------------------------------------*/
static void *g_lib = NULL;

/* Helper: resolve one symbol by its full exported name */
static void *resolve(const char *sym_name)
{
    void *p = dlsym(g_lib, sym_name);
    if (!p)
    {
        fprintf(stderr, "Missing symbol: %s  (%s)\n", sym_name, dlerror());
    }
    return p;
}

/* Load the shared library and resolve all symbols */
static int load_library(const char *path)
{
    g_lib = dlopen(path, RTLD_LAZY);
    if (!g_lib)
    {
        fprintf(stderr, "Failed to load %s: %s\n", path, dlerror());
        return -1;
    }

#define RESOLVE(var, name)                \
    do                                    \
    {                                     \
        *(void **)(&var) = resolve(name); \
        if (!var)                         \
        {                                 \
            dlclose(g_lib);               \
            g_lib = NULL;                 \
            return -1;                    \
        }                                 \
    } while (0)

    RESOLVE(fn_read, "thaiid_read");
    RESOLVE(fn_free, "thaiid_free");
    RESOLVE(fn_get_last_error, "thaiid_get_last_error");

    RESOLVE(fn_get_cid, "thaiid_get_cid");
    RESOLVE(fn_get_name_thai, "thaiid_get_name_thai");
    RESOLVE(fn_get_name_en, "thaiid_get_name_en");
    RESOLVE(fn_get_dob, "thaiid_get_dob");
    RESOLVE(fn_get_gender, "thaiid_get_gender");
    RESOLVE(fn_get_card_issuer, "thaiid_get_card_issuer");
    RESOLVE(fn_get_issue_date, "thaiid_get_issue_date");
    RESOLVE(fn_get_expire_date, "thaiid_get_expire_date");
    RESOLVE(fn_get_address, "thaiid_get_address");
    RESOLVE(fn_get_face_image, "thaiid_get_face_image");
    RESOLVE(fn_get_laser_id, "thaiid_get_laser_id");
    RESOLVE(fn_get_main_inscl, "thaiid_get_main_inscl");
    RESOLVE(fn_get_sub_inscl, "thaiid_get_sub_inscl");
    RESOLVE(fn_get_main_hospital, "thaiid_get_main_hospital");
    RESOLVE(fn_get_sub_hospital, "thaiid_get_sub_hospital");
    RESOLVE(fn_get_paid_type, "thaiid_get_paid_type");

    RESOLVE(fn_get_nhso_issue_date, "thaiid_get_nhso_issue_date");
    RESOLVE(fn_get_nhso_expire_date, "thaiid_get_nhso_expire_date");
    RESOLVE(fn_get_nhso_update_date, "thaiid_get_nhso_update_date");
    RESOLVE(fn_get_change_hospital_amount, "thaiid_get_change_hospital_amount");

#undef RESOLVE
    return 0;
}

static void unload_library(void)
{
    if (g_lib)
        dlclose(g_lib);
}

/* ---------------------------------------------------------------------------
 * Print all card data
 * -------------------------------------------------------------------------*/
static void print_card_data(const thaiid_card_data *data)
{
#define FIELD(label, getter)                                 \
    do                                                       \
    {                                                        \
        const char *val = getter(data);                      \
        printf("  %-20s %s\n", label, val ? val : "(null)"); \
    } while (0)

    printf("\n=== Personal Information ===\n");
    FIELD("CID:", fn_get_cid);
    FIELD("Name (TH):", fn_get_name_thai);
    FIELD("Name (EN):", fn_get_name_en);
    FIELD("DOB:", fn_get_dob);
    FIELD("Gender:", fn_get_gender);
    FIELD("Card Issuer:", fn_get_card_issuer);
    FIELD("Issue Date:", fn_get_issue_date);
    FIELD("Expire Date:", fn_get_expire_date);
    FIELD("Address:", fn_get_address);

    const char *face = fn_get_face_image(data);
    if (face && strlen(face) > 0)
    {
        printf("  %-20s [%zu bytes base64]\n", "Face Image:", strlen(face));
    }

    const char *laser = fn_get_laser_id(data);
    if (laser && strlen(laser) > 0)
    {
        printf("\n=== Card Info ===\n");
        FIELD("Laser ID:", fn_get_laser_id);
    }

    const char *inscl = fn_get_main_inscl(data);
    if (inscl && strlen(inscl) > 0)
    {
        printf("\n=== NHSO Information ===\n");
        FIELD("Main Inscl:", fn_get_main_inscl);
        FIELD("Sub Inscl:", fn_get_sub_inscl);
        FIELD("Main Hosp:", fn_get_main_hospital);
        FIELD("Sub Hosp:", fn_get_sub_hospital);
        FIELD("Paid Type:", fn_get_paid_type);
        FIELD("NHSO Issue:", fn_get_nhso_issue_date);
        FIELD("NHSO Expire:", fn_get_nhso_expire_date);
        FIELD("NHSO Update:", fn_get_nhso_update_date);
        FIELD("Change Hosp:", fn_get_change_hospital_amount);
    }

#undef FIELD
}

/* ---------------------------------------------------------------------------
 * Main
 * -------------------------------------------------------------------------*/
int main(int argc, char **argv)
{
    const char *libpath = "target/debug/libthaiidcard.dylib";
    const char *reader = NULL; /* NULL = auto-detect */

    if (argc > 1)
        libpath = argv[1];
    if (argc > 2)
        reader = argv[2];

    if (load_library(libpath) != 0)
    {
        return 1;
    }
    atexit(unload_library);

    printf("Using library: %s\n", libpath);
    printf("Reader:        %s\n", reader ? reader : "(auto-detect)");
    printf("Waiting for card...\n");

    /* Read: show_face=1, nhso=1, laser=0 */
    thaiid_card_data *data = fn_read(reader, 1, 1, 0);
    if (data)
    {
        print_card_data(data);
        fn_free(data);
        printf("\nDone.\n");
    }
    else
    {
        const char *err = fn_get_last_error();
        fprintf(stderr, "Error: %s\n", err ? err : "unknown error");
        return 1;
    }

    return 0;
}
