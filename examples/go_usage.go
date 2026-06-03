// go_usage.go — Example of using libthaiidcard from Go.
//
// Build the shared library first:
//     make shared
//
// Then run:
//     go run examples/go_usage.go [reader_name]

package main

/*
#cgo !windows LDFLAGS: -ldl
#include <stdlib.h>
#include <stdio.h>

// ---------------------------------------------------------------------------
// Platform abstraction for dynamic library loading
// ---------------------------------------------------------------------------
#ifdef _WIN32
#include <windows.h>
typedef HMODULE lib_handle;
static inline lib_handle lib_load(const char* name) { return LoadLibraryA(name); }
static inline void*     lib_sym(lib_handle h, const char* n) { return (void*)GetProcAddress(h, n); }
static inline int       lib_close(lib_handle h) { return FreeLibrary(h) ? 0 : -1; }
static const char*      lib_error(void) {
    static char buf[256];
    DWORD err = GetLastError();
    if (FormatMessageA(FORMAT_MESSAGE_FROM_SYSTEM, NULL, err, 0, buf, sizeof(buf), NULL))
        return buf;
    snprintf(buf, sizeof(buf), "error code %lu", (unsigned long)err);
    return buf;
}
#else
#include <dlfcn.h>
typedef void* lib_handle;
#define lib_load(name)    dlopen(name, RTLD_LAZY)
#define lib_sym(handle,n) dlsym(handle, n)
#define lib_close(handle) dlclose(handle)
#define lib_error()       dlerror()
#endif

// Opaque handle type.
typedef struct thaiid_card_data thaiid_card_data;

// Function pointer types.
typedef thaiid_card_data* (*read_fn)(const char*, int, int, int);
typedef void (*free_fn)(thaiid_card_data*);
typedef const char* (*get_last_error_fn)(void);
typedef const char* (*get_str_fn)(const thaiid_card_data*);

// Globals (populated by loadLibrary).
static lib_handle g_lib = NULL;
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

static int loadLibrary(const char* path) {
	g_lib = lib_load(path);
	if (!g_lib) {
		fprintf(stderr, "Failed to load %s: %s\n", path, lib_error());
		return -1;
	}

#define RESOLVE(var, name) do { \
	*(void**)(&var) = lib_sym(g_lib, name); \
	if (!var) { fprintf(stderr, "Missing symbol: %s (%s)\n", name, lib_error()); lib_close(g_lib); return -1; } \
} while(0)

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

static void unloadLibrary(void) {
	if (g_lib) lib_close(g_lib);
}

// Wrappers callable from Go.
thaiid_card_data* c_read(const char* reader, int face, int nhso, int laser) {
	return fn_read(reader, face, nhso, laser);
}

void c_free(thaiid_card_data* d) { fn_free(d); }

const char* c_get_last_error(void) { return fn_get_last_error(); }

const char* c_get_cid(thaiid_card_data* d) { return fn_get_cid(d); }
const char* c_get_name_thai(thaiid_card_data* d) { return fn_get_name_thai(d); }
const char* c_get_name_en(thaiid_card_data* d) { return fn_get_name_en(d); }
const char* c_get_dob(thaiid_card_data* d) { return fn_get_dob(d); }
const char* c_get_gender(thaiid_card_data* d) { return fn_get_gender(d); }
const char* c_get_card_issuer(thaiid_card_data* d) { return fn_get_card_issuer(d); }
const char* c_get_issue_date(thaiid_card_data* d) { return fn_get_issue_date(d); }
const char* c_get_expire_date(thaiid_card_data* d) { return fn_get_expire_date(d); }
const char* c_get_address(thaiid_card_data* d) { return fn_get_address(d); }
const char* c_get_face_image(thaiid_card_data* d) { return fn_get_face_image(d); }
const char* c_get_laser_id(thaiid_card_data* d) { return fn_get_laser_id(d); }
const char* c_get_main_inscl(thaiid_card_data* d) { return fn_get_main_inscl(d); }
const char* c_get_sub_inscl(thaiid_card_data* d) { return fn_get_sub_inscl(d); }
const char* c_get_main_hospital(thaiid_card_data* d) { return fn_get_main_hospital(d); }
const char* c_get_sub_hospital(thaiid_card_data* d) { return fn_get_sub_hospital(d); }
const char* c_get_paid_type(thaiid_card_data* d) { return fn_get_paid_type(d); }
const char* c_get_nhso_issue_date(thaiid_card_data* d) { return fn_get_nhso_issue_date(d); }
const char* c_get_nhso_expire_date(thaiid_card_data* d) { return fn_get_nhso_expire_date(d); }
const char* c_get_nhso_update_date(thaiid_card_data* d) { return fn_get_nhso_update_date(d); }
const char* c_get_change_hospital_amount(thaiid_card_data* d) { return fn_get_change_hospital_amount(d); }
*/
import "C"
import (
	"fmt"
	"os"
	"runtime"
)

// libraryFilename returns the shared library name for the current platform.
func libraryFilename() string {
	switch runtime.GOOS {
	case "windows":
		return "thaiidcard.dll"
	case "darwin":
		return "libthaiidcard.dylib"
	default:
		return "libthaiidcard.so"
	}
}

// findLibrary searches common locations for libthaiidcard.
func findLibrary() string {
	name := libraryFilename()

	// Relative paths (cwd = project root when running from repo root)
	relPaths := []string{
		"target/debug/",
		"target/release/",
	}
	for _, dir := range relPaths {
		path := dir + name
		if _, err := os.Stat(path); err == nil {
			return path
		}
	}

	// System-wide paths per platform
	var sysPaths []string
	switch runtime.GOOS {
	case "darwin":
		sysPaths = []string{
			"/usr/local/lib/",
			"/opt/homebrew/lib/",
		}
	case "linux":
		sysPaths = []string{
			"/usr/local/lib/",
			"/usr/lib/",
			"/usr/lib/x86_64-linux-gnu/",
			"/usr/lib/aarch64-linux-gnu/",
		}
	case "windows":
		sysRoot := os.Getenv("SYSTEMROOT")
		if sysRoot == "" {
			sysRoot = "C:\\Windows"
		}
		pf := os.Getenv("PROGRAMFILES")
		if pf == "" {
			pf = "C:\\Program Files"
		}
		sysPaths = []string{
			sysRoot + "\\System32\\",
			pf + "\\thaiidcard\\bin\\",
		}
	}
	for _, dir := range sysPaths {
		path := dir + name
		if _, err := os.Stat(path); err == nil {
			return path
		}
	}

	return ""
}

func main() {
	libPath := ""
	readerName := ""

	if len(os.Args) > 1 {
		libPath = os.Args[1]
	}
	if len(os.Args) > 2 {
		readerName = os.Args[2]
	}

	// Auto-discover if no path was given
	if libPath == "" {
		libPath = findLibrary()
		if libPath == "" {
			fmt.Fprintf(os.Stderr, "libthaiidcard not found. "+
				"Pass the path as the first argument, or build with: make shared\n")
			os.Exit(1)
		}
	}

	if C.loadLibrary(C.CString(libPath)) != 0 {
		os.Exit(1)
	}
	defer C.unloadLibrary()

	fmt.Printf("Using library: %s\n", libPath)
	fmt.Printf("Reader:        %s\n", readerName)

	var readerC *C.char
	if readerName != "" {
		readerC = C.CString(readerName)
	}

	fmt.Println("Waiting for card...")
	data := C.c_read(readerC, 1, 1, 0)
	if data == nil {
		errStr := C.c_get_last_error()
		if errStr != nil {
			fmt.Fprintf(os.Stderr, "Error: %s\n", C.GoString(errStr))
		} else {
			fmt.Fprintln(os.Stderr, "Error: unknown error")
		}
		os.Exit(1)
	}

	printCardData(data)
	C.c_free(data)
	fmt.Println("\nDone.")
}

func printCardData(data *C.thaiid_card_data) {
	cstr := func(s *C.char) string {
		if s != nil {
			return C.GoString(s)
		}
		return ""
	}

	field := func(label, val string) {
		fmt.Printf("  %-20s %s\n", label, val)
	}

	fmt.Println("\n=== Personal Information ===")
	field("CID:", cstr(C.c_get_cid(data)))
	field("Name (TH):", cstr(C.c_get_name_thai(data)))
	field("Name (EN):", cstr(C.c_get_name_en(data)))
	field("DOB:", cstr(C.c_get_dob(data)))
	field("Gender:", cstr(C.c_get_gender(data)))
	field("Card Issuer:", cstr(C.c_get_card_issuer(data)))
	field("Issue Date:", cstr(C.c_get_issue_date(data)))
	field("Expire Date:", cstr(C.c_get_expire_date(data)))
	field("Address:", cstr(C.c_get_address(data)))

	face := cstr(C.c_get_face_image(data))
	if face != "" {
		fmt.Printf("  %-20s [%d bytes base64]\n", "Face Image:", len(face))
	}

	laser := cstr(C.c_get_laser_id(data))
	if laser != "" {
		fmt.Println("\n=== Card Info ===")
		field("Laser ID:", laser)
	}

	inscl := cstr(C.c_get_main_inscl(data))
	if inscl != "" {
		fmt.Println("\n=== NHSO Information ===")
		field("Main Inscl:", inscl)
		field("Sub Inscl:", cstr(C.c_get_sub_inscl(data)))
		field("Main Hosp:", cstr(C.c_get_main_hospital(data)))
		field("Sub Hosp:", cstr(C.c_get_sub_hospital(data)))
		field("Paid Type:", cstr(C.c_get_paid_type(data)))
		field("NHSO Issue:", cstr(C.c_get_nhso_issue_date(data)))
		field("NHSO Expire:", cstr(C.c_get_nhso_expire_date(data)))
		field("NHSO Update:", cstr(C.c_get_nhso_update_date(data)))
		field("Change Hosp:", cstr(C.c_get_change_hospital_amount(data)))
	}
}
