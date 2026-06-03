#!/usr/bin/env ruby
# frozen_string_literal: true

# ruby_usage.rb — Example of using libthaiidcard from Ruby.
#
# Usage:
#   # Build the shared library first:
#   make shared
#
#   # Then run this script:
#   ruby examples/ruby_usage.rb [reader_name]

require 'fiddle'

# ---------------------------------------------------------------------------
# Platform detection and library path resolution
# ---------------------------------------------------------------------------

# Host OS detection helpers
module Platform
  WINDOWS = RbConfig::CONFIG['host_os'] =~ /mingw|mswin|cygwin/
  MACOS   = RbConfig::CONFIG['host_os'] =~ /darwin/
  LINUX   = !WINDOWS && !MACOS
end

# Return the shared library filename for the current platform.
def library_filename
  if Platform::WINDOWS
    'thaiidcard.dll'
  elsif Platform::MACOS
    'libthaiidcard.dylib'
  else
    'libthaiidcard.so'
  end
end

# Return a list of candidate paths to search for libthaiidcard.
def library_candidates
  name = library_filename
  dir  = __dir__

  candidates = [
    # Development build (run from project root)
    File.expand_path("../target/debug/#{name}", dir),
    File.expand_path("../target/release/#{name}", dir),
  ]

  if Platform::MACOS
    # macOS system-wide paths
    candidates.concat([
      "/usr/local/lib/#{name}",
      "/opt/homebrew/lib/#{name}",
    ])
  elsif Platform::LINUX
    # Linux system-wide paths
    candidates.concat([
      "/usr/local/lib/#{name}",
      "/usr/lib/#{name}",
      "/usr/lib/x86_64-linux-gnu/#{name}",
      "/usr/lib/aarch64-linux-gnu/#{name}",
    ])
  else
    # Windows system-wide paths
    sysroot = ENV['SYSTEMROOT'] || 'C:\\Windows'
    pf      = ENV['PROGRAMFILES'] || 'C:\\Program Files'
    candidates.concat([
      "#{sysroot}\\System32\\#{name}",
      "#{pf}\\thaiidcard\\bin\\#{name}",
    ])
  end

  candidates.uniq
end

# Locate libthaiidcard on the filesystem, or raise.
def find_library
  found = library_candidates.find { |p| File.exist?(p) }
  return found if found

  # Last resort: let the system linker search default paths.
  begin
    handle = Fiddle.dlopen(library_filename)
    handle.close
    return library_filename
  rescue Fiddle::DLError
    raise "libthaiidcard not found. Build it with: make shared"
  end
end

# ---------------------------------------------------------------------------
# Load library and wrap each C function with Fiddle
# ---------------------------------------------------------------------------

module ThaiIdCard
  LIB = Fiddle.dlopen(find_library)

  # Helper: define a Fiddle::Function wrapper for a C function
  def self.define_c_func(name, arg_types, ret_type)
    ptr = LIB[name]
    Fiddle::Function.new(ptr, arg_types, ret_type)
  end

  # --- thaiid_read(reader: str|null, face: int, nhso: int, laser: int) -> handle ---
  READ = define_c_func('thaiid_read',
    [Fiddle::TYPE_VOIDP, Fiddle::TYPE_INT, Fiddle::TYPE_INT, Fiddle::TYPE_INT],
    Fiddle::TYPE_VOIDP)

  # --- thaiid_free(handle) ---
  FREE = define_c_func('thaiid_free', [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)

  # --- thaiid_get_last_error() -> str ---
  GET_LAST_ERROR = define_c_func('thaiid_get_last_error', [], Fiddle::TYPE_VOIDP)

  # --- Getters: all take handle, return str ---
  GETTERS = {
    cid:                   'thaiid_get_cid',
    name_thai:             'thaiid_get_name_thai',
    name_en:               'thaiid_get_name_en',
    dob:                   'thaiid_get_dob',
    gender:                'thaiid_get_gender',
    card_issuer:           'thaiid_get_card_issuer',
    issue_date:            'thaiid_get_issue_date',
    expire_date:           'thaiid_get_expire_date',
    address:               'thaiid_get_address',
    face_image:            'thaiid_get_face_image',
    laser_id:              'thaiid_get_laser_id',
    main_inscl:            'thaiid_get_main_inscl',
    sub_inscl:             'thaiid_get_sub_inscl',
    main_hospital:         'thaiid_get_main_hospital',
    sub_hospital:          'thaiid_get_sub_hospital',
    paid_type:             'thaiid_get_paid_type',
    nhso_issue_date:       'thaiid_get_nhso_issue_date',
    nhso_expire_date:      'thaiid_get_nhso_expire_date',
    nhso_update_date:      'thaiid_get_nhso_update_date',
    change_hospital_amount: 'thaiid_get_change_hospital_amount',
  }.transform_values { |name|
    define_c_func(name, [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)
  }

  # --- Helper: read a C string pointer, return Ruby string ---
  def self.read_str(ptr)
    ptr.null? ? '' : ptr.to_s
  end

  # --- High-level reader returning a Hash ---
  def self.read_card(reader_name = nil, show_face: true, show_nhso: true, show_laser: false)
    # Fiddle handles String → C string conversion automatically.
    # nil is passed as NULL.
    handle = READ.call(reader_name, show_face ? 1 : 0, show_nhso ? 1 : 0, show_laser ? 1 : 0)

    if handle.null?
      err = read_str(GET_LAST_ERROR.call)
      raise err.empty? ? 'Unknown error' : err
    end

    begin
      data = {}
      GETTERS.each { |key, func| data[key] = read_str(func.call(handle)) }
      data
    ensure
      FREE.call(handle)
    end
  end
end

# ---------------------------------------------------------------------------
# Pretty-print
# ---------------------------------------------------------------------------

def print_card(data)
  field = ->(label, val) { puts "  #{label.ljust(20)} #{val}" }

  puts "\n=== Personal Information ==="
  field.call('CID:',         data[:cid])
  field.call('Name (TH):',   data[:name_thai])
  field.call('Name (EN):',   data[:name_en])
  field.call('DOB:',         data[:dob])
  field.call('Gender:',      data[:gender])
  field.call('Card Issuer:', data[:card_issuer])
  field.call('Issue Date:',  data[:issue_date])
  field.call('Expire Date:', data[:expire_date])
  field.call('Address:',     data[:address])

  unless data[:face_image].empty?
    puts "  #{'Face Image:'.ljust(20)} [#{data[:face_image].bytesize} bytes base64]"
  end

  unless data[:laser_id].empty?
    puts "\n=== Card Info ==="
    field.call('Laser ID:', data[:laser_id])
  end

  unless data[:main_inscl].empty?
    puts "\n=== NHSO Information ==="
    field.call('Main Inscl:',        data[:main_inscl])
    field.call('Sub Inscl:',         data[:sub_inscl])
    field.call('Main Hosp:',         data[:main_hospital])
    field.call('Sub Hosp:',          data[:sub_hospital])
    field.call('Paid Type:',         data[:paid_type])
    field.call('NHSO Issue:',        data[:nhso_issue_date])
    field.call('NHSO Expire:',       data[:nhso_expire_date])
    field.call('NHSO Update:',       data[:nhso_update_date])
    field.call('Change Hosp:',       data[:change_hospital_amount])
  end
end

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

if __FILE__ == $PROGRAM_NAME
  reader = ARGV[0] # nil means auto-detect

  puts "Using library: #{find_library}"
  puts "Reader:        #{reader || '(auto-detect)'}"
  puts 'Waiting for card...'

  begin
    data = ThaiIdCard.read_card(reader)
    print_card(data)
    puts "\nDone."
  rescue => e
    warn "Error: #{e}"
    exit 1
  end
end
