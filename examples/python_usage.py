"""
python_usage.py — Example of using libthaiidcard from Python.

Usage:
    # Build the shared library first:
    make shared

    # Then run this script:
    python3 examples/python_usage.py [reader_name]
"""

import ctypes
import ctypes.util
import os
import platform
import sys

# ---------------------------------------------------------------------------
# Locate the shared library
# ---------------------------------------------------------------------------


def find_library() -> str:
    """Locate libthaiidcard — tries platform-specific paths."""

    # Determine the library filename for the current platform
    system = platform.system()
    if system == "Windows":
        lib_name = "thaiidcard.dll"
    elif system == "Darwin":
        lib_name = "libthaiidcard.dylib"
    else:  # Linux and other Unix
        lib_name = "libthaiidcard.so"

    # Project root (two levels up from examples/)
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    candidates = [
        # Development build
        os.path.join(project_root, "target", "debug", lib_name),
        os.path.join(project_root, "target", "release", lib_name),
    ]

    if system == "Darwin":
        candidates.extend([
            "/usr/local/lib/" + lib_name,
            "/opt/homebrew/lib/" + lib_name,
        ])
    elif system == "Linux":
        candidates.extend([
            "/usr/local/lib/" + lib_name,
            "/usr/lib/" + lib_name,
            "/usr/lib/x86_64-linux-gnu/" + lib_name,
            "/usr/lib/aarch64-linux-gnu/" + lib_name,
        ])
    elif system == "Windows":
        sysroot = os.environ.get("SYSTEMROOT", "C:\\Windows")
        pf = os.environ.get("PROGRAMFILES", "C:\\Program Files")
        candidates.extend([
            os.path.join(sysroot, "System32", lib_name),
            os.path.join(pf, "thaiidcard", "bin", lib_name),
        ])

    # ctypes fallback — searches system library paths
    fallback = ctypes.util.find_library("thaiidcard")
    if fallback:
        candidates.append(fallback)

    for path in candidates:
        if path and os.path.exists(path):
            return path

    raise FileNotFoundError(
        f"libthaiidcard not found ({lib_name}). "
        f"Build it with: make shared"
    )


# ---------------------------------------------------------------------------
# Load library and define argument/return types
# ---------------------------------------------------------------------------

lib = ctypes.cdll.LoadLibrary(find_library())

# --- thaiid_read(reader: str|null, face: int, nhso: int, laser: int) -> handle ---
lib.thaiid_read.restype = ctypes.c_void_p
lib.thaiid_read.argtypes = [
    ctypes.c_char_p,
    ctypes.c_int,
    ctypes.c_int,
    ctypes.c_int,
]

# --- thaiid_free(handle) ---
lib.thaiid_free.restype = None
lib.thaiid_free.argtypes = [ctypes.c_void_p]

# --- thaiid_get_last_error() -> str ---
lib.thaiid_get_last_error.restype = ctypes.c_char_p
lib.thaiid_get_last_error.argtypes = []


# --- getters: all take handle, return str ---
def _make_getter(name: str):
    fn = getattr(lib, name)
    fn.restype = ctypes.c_char_p
    fn.argtypes = [ctypes.c_void_p]
    return fn


GETTERS = {
    "cid": _make_getter("thaiid_get_cid"),
    "name_thai": _make_getter("thaiid_get_name_thai"),
    "name_en": _make_getter("thaiid_get_name_en"),
    "dob": _make_getter("thaiid_get_dob"),
    "gender": _make_getter("thaiid_get_gender"),
    "card_issuer": _make_getter("thaiid_get_card_issuer"),
    "issue_date": _make_getter("thaiid_get_issue_date"),
    "expire_date": _make_getter("thaiid_get_expire_date"),
    "address": _make_getter("thaiid_get_address"),
    "face_image": _make_getter("thaiid_get_face_image"),
    "laser_id": _make_getter("thaiid_get_laser_id"),
    "main_inscl": _make_getter("thaiid_get_main_inscl"),
    "sub_inscl": _make_getter("thaiid_get_sub_inscl"),
    "main_hospital": _make_getter("thaiid_get_main_hospital"),
    "sub_hospital": _make_getter("thaiid_get_sub_hospital"),
    "paid_type": _make_getter("thaiid_get_paid_type"),
    "nhso_issue_date": _make_getter("thaiid_get_nhso_issue_date"),
    "nhso_expire_date": _make_getter("thaiid_get_nhso_expire_date"),
    "nhso_update_date": _make_getter("thaiid_get_nhso_update_date"),
    "change_hospital_amount": _make_getter(
        "thaiid_get_change_hospital_amount"
    ),
}


def read_card(
    reader_name: str | None = None,
    show_face: bool = True,
    show_nhso: bool = True,
    show_laser: bool = False,
) -> dict | None:
    """Read a Thai National ID smart card and return the data as a dict."""

    reader_c = reader_name.encode("utf-8") if reader_name else None
    handle = lib.thaiid_read(
        reader_c, int(show_face), int(show_nhso), int(show_laser)
    )

    if not handle:
        err = lib.thaiid_get_last_error()
        raise RuntimeError(err.decode("utf-8") if err else "Unknown error")

    try:
        data = {}
        for key, getter in GETTERS.items():
            val = getter(handle)
            data[key] = val.decode("utf-8") if val else ""
        return data
    finally:
        lib.thaiid_free(handle)


# ---------------------------------------------------------------------------
# Pretty-print
# ---------------------------------------------------------------------------


def print_card(data: dict):
    def field(label: str, key: str):
        val = data.get(key, "")
        print(f"  {label:<20} {val}")

    print("\n=== Personal Information ===")
    field("CID:", "cid")
    field("Name (TH):", "name_thai")
    field("Name (EN):", "name_en")
    field("DOB:", "dob")
    field("Gender:", "gender")
    field("Card Issuer:", "card_issuer")
    field("Issue Date:", "issue_date")
    field("Expire Date:", "expire_date")
    field("Address:", "address")

    face = data.get("face_image", "")
    if face:
        print(f"  {'Face Image:':<20} [{len(face)} bytes base64]")

    laser = data.get("laser_id", "")
    if laser:
        print("\n=== Card Info ===")
        field("Laser ID:", "laser_id")

    inscl = data.get("main_inscl", "")
    if inscl:
        print("\n=== NHSO Information ===")
        field("Main Inscl:", "main_inscl")
        field("Sub Inscl:", "sub_inscl")
        field("Main Hosp:", "main_hospital")
        field("Sub Hosp:", "sub_hospital")
        field("Paid Type:", "paid_type")
        field("NHSO Issue:", "nhso_issue_date")
        field("NHSO Expire:", "nhso_expire_date")
        field("NHSO Update:", "nhso_update_date")
        field("Change Hosp:", "change_hospital_amount")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    reader = sys.argv[1] if len(sys.argv) > 1 else None

    print(f"Using library: {find_library()}")
    print(f"Reader:        {reader or '(auto-detect)'}")
    print("Waiting for card...")

    try:
        data = read_card(reader)
        print_card(data)
        print("\nDone.")
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
