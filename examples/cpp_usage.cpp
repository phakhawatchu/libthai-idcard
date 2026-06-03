/**
 * cpp_usage.cpp — Example of using libthaiidcard from C++.
 *
 * Supports macOS, Linux, and Windows.
 *
 * Compile (dynamic loading):
 *   macOS / Linux:   g++ -std=c++17 -o cpp_usage examples/cpp_usage.cpp -ldl
 *   Windows (MSVC):  cl /EHsc examples/cpp_usage.cpp
 *   Windows (MinGW): g++ -std=c++17 -o cpp_usage.exe examples/cpp_usage.cpp
 *   ./cpp_usage
 *
 * Compile-time linking (macOS/Linux):
 *   g++ -std=c++17 -o cpp_usage examples/cpp_usage.cpp \
 *       -Ltarget/debug -lthaiidcard -lpcsclite -Wl,-rpath,target/debug
 *   ./cpp_usage
 */

#include <cstdio>
#include <cstring>
#include <iostream>
#include <memory>
#include <string>

/* ---------------------------------------------------------------------------
 * Platform abstraction for dynamic library loading
 * -------------------------------------------------------------------------*/
#ifdef _WIN32
#include <windows.h>
using LibHandle = HMODULE;
#define LIB_LOAD(name)    LoadLibraryA(name)
#define LIB_SYM(handle,n) GetProcAddress(handle, n)
#define LIB_CLOSE(handle) FreeLibrary(handle)
static std::string last_lib_error()
{
    static char buf[256];
    DWORD err = GetLastError();
    if (FormatMessageA(FORMAT_MESSAGE_FROM_SYSTEM, NULL, err, 0,
                       buf, sizeof(buf), NULL))
        return buf;
    return "error code " + std::to_string(err);
}
#else
#include <dlfcn.h>
using LibHandle = void *;
#define LIB_LOAD(name)    dlopen(name, RTLD_LAZY)
#define LIB_SYM(handle,n) dlsym(handle, n)
#define LIB_CLOSE(handle) dlclose(handle)
static std::string last_lib_error() { return dlerror(); }
#endif

/* ---------------------------------------------------------------------------
 * Opaque handle type
 * -------------------------------------------------------------------------*/
using ThaiIdHandle = struct thaiid_card_data;

/* ---------------------------------------------------------------------------
 * RAII wrapper for the shared library
 * -------------------------------------------------------------------------*/
class DynamicLibrary
{
    LibHandle handle_;

public:
    DynamicLibrary() : handle_(nullptr) {}

    explicit DynamicLibrary(const std::string &path)
        : handle_(LIB_LOAD(path.c_str()))
    {
        if (!handle_)
            throw std::runtime_error("Failed to load " + path + ": " + last_lib_error());
    }

    ~DynamicLibrary()
    {
        if (handle_)
            LIB_CLOSE(handle_);
    }

    DynamicLibrary(const DynamicLibrary &) = delete;
    DynamicLibrary &operator=(const DynamicLibrary &) = delete;

    DynamicLibrary(DynamicLibrary &&other) noexcept : handle_(other.handle_)
    {
        other.handle_ = nullptr;
    }
    DynamicLibrary &operator=(DynamicLibrary &&other) noexcept
    {
        if (this != &other)
        {
            if (handle_)
                LIB_CLOSE(handle_);
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    /** Resolve a symbol from the library. Returns nullptr on failure. */
    template <typename T>
    T resolve(const char *name) const
    {
        auto *ptr = LIB_SYM(handle_, name);
        if (!ptr)
            throw std::runtime_error(std::string("Missing symbol: ") + name + " (" + last_lib_error() + ")");
        return reinterpret_cast<T>(ptr);
    }
};

/* ---------------------------------------------------------------------------
 * Function pointer types for the C API
 * -------------------------------------------------------------------------*/
using ReadFn       = ThaiIdHandle *(*)(const char *, int, int, int);
using FreeFn       = void (*)(ThaiIdHandle *);
using GetErrorFn   = const char *(*)();
using GetStrFn     = const char *(*)(const ThaiIdHandle *);

/* ---------------------------------------------------------------------------
 * High-level wrapper that owns a card data handle and provides typed accessors
 * -------------------------------------------------------------------------*/
class CardData
{
    ThaiIdHandle *handle_;
    FreeFn free_;

    GetStrFn get_cid_;
    GetStrFn get_name_thai_;
    GetStrFn get_name_en_;
    GetStrFn get_dob_;
    GetStrFn get_gender_;
    GetStrFn get_card_issuer_;
    GetStrFn get_issue_date_;
    GetStrFn get_expire_date_;
    GetStrFn get_address_;
    GetStrFn get_face_image_;
    GetStrFn get_laser_id_;
    GetStrFn get_main_inscl_;
    GetStrFn get_sub_inscl_;
    GetStrFn get_main_hospital_;
    GetStrFn get_sub_hospital_;
    GetStrFn get_paid_type_;
    GetStrFn get_nhso_issue_date_;
    GetStrFn get_nhso_expire_date_;
    GetStrFn get_nhso_update_date_;
    GetStrFn get_change_hospital_amount_;

    static std::string cstr(const char *s) { return s ? s : ""; }

public:
    CardData(ThaiIdHandle *handle, FreeFn free_fn, GetStrFn *getters)
        : handle_(handle), free_(free_fn),
          get_cid_(getters[0]), get_name_thai_(getters[1]),
          get_name_en_(getters[2]), get_dob_(getters[3]),
          get_gender_(getters[4]), get_card_issuer_(getters[5]),
          get_issue_date_(getters[6]), get_expire_date_(getters[7]),
          get_address_(getters[8]), get_face_image_(getters[9]),
          get_laser_id_(getters[10]), get_main_inscl_(getters[11]),
          get_sub_inscl_(getters[12]), get_main_hospital_(getters[13]),
          get_sub_hospital_(getters[14]), get_paid_type_(getters[15]),
          get_nhso_issue_date_(getters[16]), get_nhso_expire_date_(getters[17]),
          get_nhso_update_date_(getters[18]), get_change_hospital_amount_(getters[19])
    {}

    ~CardData()
    {
        if (handle_)
            free_(handle_);
    }

    CardData(const CardData &) = delete;
    CardData &operator=(const CardData &) = delete;
    CardData(CardData &&other) noexcept
        : handle_(other.handle_), free_(other.free_),
          get_cid_(other.get_cid_), get_name_thai_(other.get_name_thai_),
          get_name_en_(other.get_name_en_), get_dob_(other.get_dob_),
          get_gender_(other.get_gender_), get_card_issuer_(other.get_card_issuer_),
          get_issue_date_(other.get_issue_date_), get_expire_date_(other.get_expire_date_),
          get_address_(other.get_address_), get_face_image_(other.get_face_image_),
          get_laser_id_(other.get_laser_id_), get_main_inscl_(other.get_main_inscl_),
          get_sub_inscl_(other.get_sub_inscl_), get_main_hospital_(other.get_main_hospital_),
          get_sub_hospital_(other.get_sub_hospital_), get_paid_type_(other.get_paid_type_),
          get_nhso_issue_date_(other.get_nhso_issue_date_),
          get_nhso_expire_date_(other.get_nhso_expire_date_),
          get_nhso_update_date_(other.get_nhso_update_date_),
          get_change_hospital_amount_(other.get_change_hospital_amount_)
    {
        other.handle_ = nullptr;
    }
    CardData &operator=(CardData &&other) noexcept
    {
        if (this != &other)
        {
            if (handle_)
                free_(handle_);
            handle_ = other.handle_;
            other.handle_ = nullptr;
            free_ = other.free_;
            get_cid_ = other.get_cid_;
            get_name_thai_ = other.get_name_thai_;
            get_name_en_ = other.get_name_en_;
            get_dob_ = other.get_dob_;
            get_gender_ = other.get_gender_;
            get_card_issuer_ = other.get_card_issuer_;
            get_issue_date_ = other.get_issue_date_;
            get_expire_date_ = other.get_expire_date_;
            get_address_ = other.get_address_;
            get_face_image_ = other.get_face_image_;
            get_laser_id_ = other.get_laser_id_;
            get_main_inscl_ = other.get_main_inscl_;
            get_sub_inscl_ = other.get_sub_inscl_;
            get_main_hospital_ = other.get_main_hospital_;
            get_sub_hospital_ = other.get_sub_hospital_;
            get_paid_type_ = other.get_paid_type_;
            get_nhso_issue_date_ = other.get_nhso_issue_date_;
            get_nhso_expire_date_ = other.get_nhso_expire_date_;
            get_nhso_update_date_ = other.get_nhso_update_date_;
            get_change_hospital_amount_ = other.get_change_hospital_amount_;
        }
        return *this;
    }

    // --- Accessors ---------------------------------------------------------
    std::string cid()                   const { return cstr(get_cid_(handle_)); }
    std::string name_thai()             const { return cstr(get_name_thai_(handle_)); }
    std::string name_en()               const { return cstr(get_name_en_(handle_)); }
    std::string dob()                   const { return cstr(get_dob_(handle_)); }
    std::string gender()                const { return cstr(get_gender_(handle_)); }
    std::string card_issuer()           const { return cstr(get_card_issuer_(handle_)); }
    std::string issue_date()            const { return cstr(get_issue_date_(handle_)); }
    std::string expire_date()           const { return cstr(get_expire_date_(handle_)); }
    std::string address()               const { return cstr(get_address_(handle_)); }
    std::string face_image()            const { return cstr(get_face_image_(handle_)); }
    std::string laser_id()              const { return cstr(get_laser_id_(handle_)); }
    std::string main_inscl()            const { return cstr(get_main_inscl_(handle_)); }
    std::string sub_inscl()             const { return cstr(get_sub_inscl_(handle_)); }
    std::string main_hospital()         const { return cstr(get_main_hospital_(handle_)); }
    std::string sub_hospital()          const { return cstr(get_sub_hospital_(handle_)); }
    std::string paid_type()             const { return cstr(get_paid_type_(handle_)); }
    std::string nhso_issue_date()       const { return cstr(get_nhso_issue_date_(handle_)); }
    std::string nhso_expire_date()      const { return cstr(get_nhso_expire_date_(handle_)); }
    std::string nhso_update_date()      const { return cstr(get_nhso_update_date_(handle_)); }
    std::string change_hospital_amount() const { return cstr(get_change_hospital_amount_(handle_)); }
};

/* ---------------------------------------------------------------------------
 * Auto-discover the shared library path
 * -------------------------------------------------------------------------*/
static std::string library_filename()
{
#if defined(_WIN32)
    return "thaiidcard.dll";
#elif defined(__APPLE__)
    return "libthaiidcard.dylib";
#else
    return "libthaiidcard.so";
#endif
}

static std::string find_library()
{
    auto name = library_filename();

    /* Relative paths (cwd = project root when running from repo root) */
    const char *relative[] = { "target/debug/", "target/release/", nullptr };
    for (int i = 0; relative[i]; ++i)
    {
        auto path = std::string(relative[i]) + name;
        if (FILE *f = std::fopen(path.c_str(), "r"))
        {
            std::fclose(f);
            return path;
        }
    }

#ifdef __APPLE__
    const char *syspaths[] = { "/usr/local/lib/", "/opt/homebrew/lib/", nullptr };
#elif defined(_WIN32)
    const char *syspaths[] = { nullptr };  // pass manually on Windows
#else
    const char *syspaths[] = {
        "/usr/local/lib/", "/usr/lib/",
        "/usr/lib/x86_64-linux-gnu/", "/usr/lib/aarch64-linux-gnu/",
        nullptr
    };
#endif

#ifndef _WIN32
    for (int i = 0; syspaths[i]; ++i)
    {
        auto path = std::string(syspaths[i]) + name;
        if (FILE *f = std::fopen(path.c_str(), "r"))
        {
            std::fclose(f);
            return path;
        }
    }
#endif

    throw std::runtime_error(
        "libthaiidcard not found. Pass the path as the first argument, "
        "or build with: make shared");
}

/* ---------------------------------------------------------------------------
 * Print card data
 * -------------------------------------------------------------------------*/
static void print_card(const CardData &data)
{
    auto field = [](const char *label, const std::string &val)
    {
        printf("  %-20s %s\n", label, val.c_str());
    };

    auto face  = data.face_image();
    auto laser = data.laser_id();
    auto inscl = data.main_inscl();

    printf("\n=== Personal Information ===\n");
    field("CID:",         data.cid());
    field("Name (TH):",   data.name_thai());
    field("Name (EN):",   data.name_en());
    field("DOB:",         data.dob());
    field("Gender:",      data.gender());
    field("Card Issuer:", data.card_issuer());
    field("Issue Date:",  data.issue_date());
    field("Expire Date:", data.expire_date());
    field("Address:",     data.address());

    if (!face.empty())
        printf("  %-20s [%zu bytes base64]\n", "Face Image:", face.size());

    if (!laser.empty())
    {
        printf("\n=== Card Info ===\n");
        field("Laser ID:", laser);
    }

    if (!inscl.empty())
    {
        printf("\n=== NHSO Information ===\n");
        field("Main Inscl:",        data.main_inscl());
        field("Sub Inscl:",         data.sub_inscl());
        field("Main Hosp:",         data.main_hospital());
        field("Sub Hosp:",          data.sub_hospital());
        field("Paid Type:",         data.paid_type());
        field("NHSO Issue:",        data.nhso_issue_date());
        field("NHSO Expire:",       data.nhso_expire_date());
        field("NHSO Update:",       data.nhso_update_date());
        field("Change Hosp:",       data.change_hospital_amount());
    }
}

/* ---------------------------------------------------------------------------
 * Main
 * -------------------------------------------------------------------------*/
int main(int argc, char **argv)
{
    const char *libpath = nullptr;
    const char *reader  = nullptr;  /* nullptr = auto-detect */

    if (argc > 1) libpath = argv[1];
    if (argc > 2) reader  = argv[2];

    try
    {
        std::string found_path;
        if (!libpath)
        {
            found_path = find_library();
            libpath = found_path.c_str();
        }

        printf("Using library: %s\n", libpath);
        printf("Reader:        %s\n", reader ? reader : "(auto-detect)");

        DynamicLibrary lib(libpath);

        /* Resolve all symbols */
        auto read_fn        = lib.resolve<ReadFn>("thaiid_read");
        auto free_fn        = lib.resolve<FreeFn>("thaiid_free");
        auto get_last_error = lib.resolve<GetErrorFn>("thaiid_get_last_error");

        GetStrFn getters[20] = {
            lib.resolve<GetStrFn>("thaiid_get_cid"),
            lib.resolve<GetStrFn>("thaiid_get_name_thai"),
            lib.resolve<GetStrFn>("thaiid_get_name_en"),
            lib.resolve<GetStrFn>("thaiid_get_dob"),
            lib.resolve<GetStrFn>("thaiid_get_gender"),
            lib.resolve<GetStrFn>("thaiid_get_card_issuer"),
            lib.resolve<GetStrFn>("thaiid_get_issue_date"),
            lib.resolve<GetStrFn>("thaiid_get_expire_date"),
            lib.resolve<GetStrFn>("thaiid_get_address"),
            lib.resolve<GetStrFn>("thaiid_get_face_image"),
            lib.resolve<GetStrFn>("thaiid_get_laser_id"),
            lib.resolve<GetStrFn>("thaiid_get_main_inscl"),
            lib.resolve<GetStrFn>("thaiid_get_sub_inscl"),
            lib.resolve<GetStrFn>("thaiid_get_main_hospital"),
            lib.resolve<GetStrFn>("thaiid_get_sub_hospital"),
            lib.resolve<GetStrFn>("thaiid_get_paid_type"),
            lib.resolve<GetStrFn>("thaiid_get_nhso_issue_date"),
            lib.resolve<GetStrFn>("thaiid_get_nhso_expire_date"),
            lib.resolve<GetStrFn>("thaiid_get_nhso_update_date"),
            lib.resolve<GetStrFn>("thaiid_get_change_hospital_amount"),
        };

        printf("Waiting for card...\n");

        auto *handle = read_fn(reader, 1, 1, 0);
        if (!handle)
        {
            auto err = get_last_error();
            throw std::runtime_error(err ? err : "Unknown error");
        }

        {
            CardData data(handle, free_fn, getters);
            print_card(data);
        }  /* CardData destructor calls thaiid_free */

        printf("\nDone.\n");
    }
    catch (const std::exception &e)
    {
        fprintf(stderr, "Error: %s\n", e.what());
        return 1;
    }

    return 0;
}
