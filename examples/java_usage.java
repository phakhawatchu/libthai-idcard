///usr/bin/env jbang "$0" "$@" ; exit $?
//DEPS net.java.dev.jna:jna:5.19.0

/**
 * java_usage.java — Example of using libthaiidcard from Java via JNA.
 *
 * Usage:
 *   # Build the shared library first:
 *   make shared
 *
 *   # Run with jbang (auto-downloads JNA, no manual setup):
 *   jbang examples/java_usage.java [reader_name]
 *
 *   # Or compile & run manually (download jna.jar first):
 *   javac -cp jna.jar examples/java_usage.java
 *   java -cp .:jna.jar java_usage [reader_name]
 *
 * JNA download: https://github.com/java-native-access/jna#download
 */

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import java.io.File;

public class java_usage {

    // -----------------------------------------------------------------------
    // JNA binding to the C API
    // -----------------------------------------------------------------------
    public interface ThaiIdCard extends Library {
        Pointer thaiid_read(String reader, int face, int nhso, int laser);
        void    thaiid_free(Pointer data);
        String  thaiid_get_last_error();

        String thaiid_get_cid(Pointer data);
        String thaiid_get_name_thai(Pointer data);
        String thaiid_get_name_en(Pointer data);
        String thaiid_get_dob(Pointer data);
        String thaiid_get_gender(Pointer data);
        String thaiid_get_card_issuer(Pointer data);
        String thaiid_get_issue_date(Pointer data);
        String thaiid_get_expire_date(Pointer data);
        String thaiid_get_address(Pointer data);
        String thaiid_get_face_image(Pointer data);
        String thaiid_get_laser_id(Pointer data);
        String thaiid_get_main_inscl(Pointer data);
        String thaiid_get_sub_inscl(Pointer data);
        String thaiid_get_main_hospital(Pointer data);
        String thaiid_get_sub_hospital(Pointer data);
        String thaiid_get_paid_type(Pointer data);
        String thaiid_get_nhso_issue_date(Pointer data);
        String thaiid_get_nhso_expire_date(Pointer data);
        String thaiid_get_nhso_update_date(Pointer data);
        String thaiid_get_change_hospital_amount(Pointer data);
    }

    // -----------------------------------------------------------------------
    // Library discovery
    // -----------------------------------------------------------------------
    private static String findLibrary() {
        String os = System.getProperty("os.name").toLowerCase();
        String libName;
        if (os.contains("win")) {
            libName = "thaiidcard.dll";
        } else if (os.contains("mac")) {
            libName = "libthaiidcard.dylib";
        } else {
            libName = "libthaiidcard.so";
        }

        // Derive project root: this class is at examples/java_usage.java
        String classPath = new File("examples/java_usage.java").getAbsolutePath();
        String projectRoot = new File(classPath).getParentFile().getParent();

        String[][] dirs = {
            { projectRoot + "/target/debug/",  libName },
            { projectRoot + "/target/release/", libName },
            // System paths per platform
        };

        // Platform-specific system paths
        String[] sysDirs;
        if (os.contains("mac")) {
            sysDirs = new String[]{ "/usr/local/lib/", "/opt/homebrew/lib/" };
        } else if (os.contains("win")) {
            String sysRoot = System.getenv("SYSTEMROOT");
            if (sysRoot == null) sysRoot = "C:\\Windows";
            String pf = System.getenv("PROGRAMFILES");
            if (pf == null) pf = "C:\\Program Files";
            sysDirs = new String[]{ sysRoot + "\\System32\\", pf + "\\thaiidcard\\bin\\" };
        } else {
            sysDirs = new String[]{
                "/usr/local/lib/", "/usr/lib/",
                "/usr/lib/x86_64-linux-gnu/", "/usr/lib/aarch64-linux-gnu/"
            };
        }

        for (String dir : sysDirs) {
            String path = dir + libName;
            if (new File(path).exists()) {
                return path;
            }
        }

        // Development build paths
        for (String[] pair : dirs) {
            String path = pair[0] + pair[1];
            if (new File(path).exists()) {
                return path;
            }
        }

        // Fallback: let JNA search system library paths
        return libName;
    }

    // -----------------------------------------------------------------------
    // Read card data
    // -----------------------------------------------------------------------
    private static Pointer readCard(ThaiIdCard lib, String reader,
                                     boolean showFace, boolean showNhso, boolean showLaser) {
        Pointer handle = lib.thaiid_read(reader,
            showFace ? 1 : 0, showNhso ? 1 : 0, showLaser ? 1 : 0);
        if (handle == null) {
            String err = lib.thaiid_get_last_error();
            throw new RuntimeException(err != null ? err : "Unknown error");
        }
        return handle;
    }

    // -----------------------------------------------------------------------
    // Pretty-print
    // -----------------------------------------------------------------------
    private static String safe(String s) {
        return s != null ? s : "";
    }

    private static void printCard(ThaiIdCard lib, Pointer data) {
        String face  = safe(lib.thaiid_get_face_image(data));
        String laser = safe(lib.thaiid_get_laser_id(data));
        String inscl = safe(lib.thaiid_get_main_inscl(data));

        System.out.println("\n=== Personal Information ===");
        System.out.printf("  %-20s %s%n", "CID:",         safe(lib.thaiid_get_cid(data)));
        System.out.printf("  %-20s %s%n", "Name (TH):",   safe(lib.thaiid_get_name_thai(data)));
        System.out.printf("  %-20s %s%n", "Name (EN):",   safe(lib.thaiid_get_name_en(data)));
        System.out.printf("  %-20s %s%n", "DOB:",         safe(lib.thaiid_get_dob(data)));
        System.out.printf("  %-20s %s%n", "Gender:",      safe(lib.thaiid_get_gender(data)));
        System.out.printf("  %-20s %s%n", "Card Issuer:", safe(lib.thaiid_get_card_issuer(data)));
        System.out.printf("  %-20s %s%n", "Issue Date:",  safe(lib.thaiid_get_issue_date(data)));
        System.out.printf("  %-20s %s%n", "Expire Date:", safe(lib.thaiid_get_expire_date(data)));
        System.out.printf("  %-20s %s%n", "Address:",     safe(lib.thaiid_get_address(data)));

        if (!face.isEmpty()) {
            System.out.printf("  %-20s [%d bytes base64]%n", "Face Image:", face.length());
        }

        if (!laser.isEmpty()) {
            System.out.println("\n=== Card Info ===");
            System.out.printf("  %-20s %s%n", "Laser ID:", laser);
        }

        if (!inscl.isEmpty()) {
            System.out.println("\n=== NHSO Information ===");
            System.out.printf("  %-20s %s%n", "Main Inscl:",  safe(lib.thaiid_get_main_inscl(data)));
            System.out.printf("  %-20s %s%n", "Sub Inscl:",   safe(lib.thaiid_get_sub_inscl(data)));
            System.out.printf("  %-20s %s%n", "Main Hosp:",   safe(lib.thaiid_get_main_hospital(data)));
            System.out.printf("  %-20s %s%n", "Sub Hosp:",    safe(lib.thaiid_get_sub_hospital(data)));
            System.out.printf("  %-20s %s%n", "Paid Type:",   safe(lib.thaiid_get_paid_type(data)));
            System.out.printf("  %-20s %s%n", "NHSO Issue:",  safe(lib.thaiid_get_nhso_issue_date(data)));
            System.out.printf("  %-20s %s%n", "NHSO Expire:", safe(lib.thaiid_get_nhso_expire_date(data)));
            System.out.printf("  %-20s %s%n", "NHSO Update:", safe(lib.thaiid_get_nhso_update_date(data)));
            System.out.printf("  %-20s %s%n", "Change Hosp:", safe(lib.thaiid_get_change_hospital_amount(data)));
        }
    }

    // -----------------------------------------------------------------------
    // Main
    // -----------------------------------------------------------------------
    public static void main(String[] args) {
        String reader = args.length > 0 ? args[0] : null;

        String libPath = findLibrary();
        System.out.println("Using library: " + libPath);
        System.out.println("Reader:        " + (reader != null ? reader : "(auto-detect)"));
        System.out.println("Waiting for card...");

        try {
            ThaiIdCard lib = Native.load(libPath, ThaiIdCard.class);
            Pointer data = readCard(lib, reader, true, true, false);
            try {
                printCard(lib, data);
                System.out.println("\nDone.");
            } finally {
                lib.thaiid_free(data);
            }
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
            System.exit(1);
        }
    }
}
