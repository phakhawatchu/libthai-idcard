///usr/bin/env jbang "$0" "$@" ; exit $?
//DEPS net.java.dev.jna:jna:5.19.0

/**
 * kotlin_usage.kt — Example of using libthaiidcard from Kotlin via JNA.
 *
 * Usage:
 *   # Build the shared library first:
 *   make shared
 *
 *   # Run with jbang (auto-downloads JNA, no manual setup):
 *   jbang examples/kotlin_usage.kt [reader_name]
 *
 *   # Or compile & run manually (download jna.jar first):
 *   kotlinc -cp jna.jar examples/kotlin_usage.kt
 *   kotlin -cp .:jna.jar kotlin_usageKt [reader_name]
 *
 * JNA download: https://github.com/java-native-access/jna#download
 */

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import java.io.File

// -----------------------------------------------------------------------
// JNA binding to the C API
// -----------------------------------------------------------------------
interface ThaiIdCard : Library {
    fun thaiid_read(reader: String?, face: Int, nhso: Int, laser: Int): Pointer?
    fun thaiid_free(data: Pointer?)
    fun thaiid_get_last_error(): String?

    fun thaiid_get_cid(data: Pointer?): String?
    fun thaiid_get_name_thai(data: Pointer?): String?
    fun thaiid_get_name_en(data: Pointer?): String?
    fun thaiid_get_dob(data: Pointer?): String?
    fun thaiid_get_gender(data: Pointer?): String?
    fun thaiid_get_card_issuer(data: Pointer?): String?
    fun thaiid_get_issue_date(data: Pointer?): String?
    fun thaiid_get_expire_date(data: Pointer?): String?
    fun thaiid_get_address(data: Pointer?): String?
    fun thaiid_get_face_image(data: Pointer?): String?
    fun thaiid_get_laser_id(data: Pointer?): String?
    fun thaiid_get_main_inscl(data: Pointer?): String?
    fun thaiid_get_sub_inscl(data: Pointer?): String?
    fun thaiid_get_main_hospital(data: Pointer?): String?
    fun thaiid_get_sub_hospital(data: Pointer?): String?
    fun thaiid_get_paid_type(data: Pointer?): String?
    fun thaiid_get_nhso_issue_date(data: Pointer?): String?
    fun thaiid_get_nhso_expire_date(data: Pointer?): String?
    fun thaiid_get_nhso_update_date(data: Pointer?): String?
    fun thaiid_get_change_hospital_amount(data: Pointer?): String?
}

// -----------------------------------------------------------------------
// Library discovery
// -----------------------------------------------------------------------
fun findLibrary(): String {
    val os = System.getProperty("os.name").lowercase()
    val libName = when {
        os.contains("win") -> "thaiidcard.dll"
        os.contains("mac") -> "libthaiidcard.dylib"
        else               -> "libthaiidcard.so"
    }

    // Derive project root: this script is at examples/kotlin_usage.kt
    val scriptFile = File("examples/kotlin_usage.kt").absoluteFile
    val projectRoot = scriptFile.parentFile.parent

    // Development build paths
    for (dir in listOf("target/debug/", "target/release/")) {
        val path = "${projectRoot}/${dir}${libName}"
        if (File(path).exists()) return path
    }

    // Platform-specific system paths
    val sysDirs = when {
        os.contains("mac") -> listOf("/usr/local/lib/", "/opt/homebrew/lib/")
        os.contains("win") -> {
            val sysRoot = System.getenv("SYSTEMROOT") ?: "C:\\Windows"
            val pf = System.getenv("PROGRAMFILES") ?: "C:\\Program Files"
            listOf("${sysRoot}\\System32\\", "${pf}\\thaiidcard\\bin\\")
        }
        else -> listOf(
            "/usr/local/lib/", "/usr/lib/",
            "/usr/lib/x86_64-linux-gnu/", "/usr/lib/aarch64-linux-gnu/"
        )
    }

    for (dir in sysDirs) {
        val path = dir + libName
        if (File(path).exists()) return path
    }

    // Fallback: let JNA search system library paths
    return libName
}

// -----------------------------------------------------------------------
// Read card data
// -----------------------------------------------------------------------
fun readCard(lib: ThaiIdCard, reader: String?,
             showFace: Boolean, showNhso: Boolean, showLaser: Boolean): Pointer {
    val handle = lib.thaiid_read(reader,
        if (showFace) 1 else 0,
        if (showNhso) 1 else 0,
        if (showLaser) 1 else 0)
    if (handle == null) {
        val err = lib.thaiid_get_last_error()
        throw RuntimeException(err ?: "Unknown error")
    }
    return handle
}

// -----------------------------------------------------------------------
// Pretty-print
// -----------------------------------------------------------------------
fun printCard(lib: ThaiIdCard, data: Pointer) {
    fun String?.safe() = this ?: ""
    val face  = lib.thaiid_get_face_image(data).safe()
    val laser = lib.thaiid_get_laser_id(data).safe()
    val inscl = lib.thaiid_get_main_inscl(data).safe()

    println("\n=== Personal Information ===")
    System.out.printf("  %-20s %s%n", "CID:",         lib.thaiid_get_cid(data).safe())
    System.out.printf("  %-20s %s%n", "Name (TH):",   lib.thaiid_get_name_thai(data).safe())
    System.out.printf("  %-20s %s%n", "Name (EN):",   lib.thaiid_get_name_en(data).safe())
    System.out.printf("  %-20s %s%n", "DOB:",         lib.thaiid_get_dob(data).safe())
    System.out.printf("  %-20s %s%n", "Gender:",      lib.thaiid_get_gender(data).safe())
    System.out.printf("  %-20s %s%n", "Card Issuer:", lib.thaiid_get_card_issuer(data).safe())
    System.out.printf("  %-20s %s%n", "Issue Date:",  lib.thaiid_get_issue_date(data).safe())
    System.out.printf("  %-20s %s%n", "Expire Date:", lib.thaiid_get_expire_date(data).safe())
    System.out.printf("  %-20s %s%n", "Address:",     lib.thaiid_get_address(data).safe())

    if (face.isNotEmpty()) {
        System.out.printf("  %-20s [%d bytes base64]%n", "Face Image:", face.length)
    }

    if (laser.isNotEmpty()) {
        println("\n=== Card Info ===")
        System.out.printf("  %-20s %s%n", "Laser ID:", laser)
    }

    if (inscl.isNotEmpty()) {
        println("\n=== NHSO Information ===")
        System.out.printf("  %-20s %s%n", "Main Inscl:",  lib.thaiid_get_main_inscl(data).safe())
        System.out.printf("  %-20s %s%n", "Sub Inscl:",   lib.thaiid_get_sub_inscl(data).safe())
        System.out.printf("  %-20s %s%n", "Main Hosp:",   lib.thaiid_get_main_hospital(data).safe())
        System.out.printf("  %-20s %s%n", "Sub Hosp:",    lib.thaiid_get_sub_hospital(data).safe())
        System.out.printf("  %-20s %s%n", "Paid Type:",   lib.thaiid_get_paid_type(data).safe())
        System.out.printf("  %-20s %s%n", "NHSO Issue:",  lib.thaiid_get_nhso_issue_date(data).safe())
        System.out.printf("  %-20s %s%n", "NHSO Expire:", lib.thaiid_get_nhso_expire_date(data).safe())
        System.out.printf("  %-20s %s%n", "NHSO Update:", lib.thaiid_get_nhso_update_date(data).safe())
        System.out.printf("  %-20s %s%n", "Change Hosp:", lib.thaiid_get_change_hospital_amount(data).safe())
    }
}

// -----------------------------------------------------------------------
// Main
// -----------------------------------------------------------------------
fun main(args: Array<String>) {
    val reader = args.firstOrNull()

    val libPath = findLibrary()
    println("Using library: $libPath")
    println("Reader:        ${reader ?: "(auto-detect)"}")
    println("Waiting for card...")

    try {
        val lib = Native.load(libPath, ThaiIdCard::class.java)
        val data = readCard(lib, reader, true, true, false)
        try {
            printCard(lib, data)
            println("\nDone.")
        } finally {
            lib.thaiid_free(data)
        }
    } catch (e: Exception) {
        System.err.println("Error: ${e.message}")
        kotlin.system.exitProcess(1)
    }
}
