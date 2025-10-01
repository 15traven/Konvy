use phf::phf_map;

pub static FACTORS: phf::Map<&'static str, f64> = phf_map! {
    // Mass
    "kg" => 1.0,
    "g" => 0.001,
    "mg" => 1e-6,
    "t" => 1000.0,
    "oz" => 0.0283495,
    "lb" => 0.453592,
    "stone" => 6.35029,
    "ct" => 0.0002,

    // Length
    "m" => 1.0,
    "km" => 1000.0,
    "cm" => 0.01,
    "mm" => 0.001,
    "inch" => 0.0254,
    "ft" => 0.3048,
    "yard" => 0.9144,
    "mile" => 1609.344,
    "NM" => 1852.0,

    // Angle
    "rad" => 1.0,
    "deg" => std::f64::consts::PI / 180.0,
    "grad" => std::f64::consts::PI / 200.0,
    "mrad" => 0.001,
    "arcmin" => std::f64::consts::PI / 10_800.0,
    "arcseconds" => std::f64::consts::PI / 648_000.0,

    // Area
    "m2" => 1.0,
    "km2" => 1_000_000.0,
    "acre" => 4046.8564224,
    "hectare" => 10_000.0,
    "feet2" => 0.09290304,
    "inch2" => 0.00064516,
    "mile2" => 2_589_988.110336,
    "yard2" => 0.83612736,

    // Volume
    "l" => 1.0,
    "ml" => 0.001,
    "kl" => 1000.0,
    "m3" => 1000.0,
    "inch3" => 0.0163871,
    "feet3" => 28.3168,
    "pt" => 0.568261,
    "qt" => 1.13652,
    "gal" => 4.54609,
    "uscup" => 0.24,
    "usfloz" => 0.0295735,
    "uspt" => 0.473176,
    "usqt" => 0.946353,
    "usgal" => 3.78541,
    "ustbsp" => 0.0147868,
    "ustsp" => 0.00492892,
    "floz" => 0.0284131,
    "tbsp" => 0.0177582,
    "tsp" => 0.00591939,
    "oilbarrel" => 158.987,

    // Speed
    "mps" => 1.0,
    "fps" => 0.3048,
    "kph" => 0.277778,
    "mph" => 0.44704,
    "knot" => 0.514444,

    // Pressure
    "Pa" => 1.0,
    "atm" => 101325.0,
    "bar" => 100000.0,
    "psi" => 6894.757,
    "Torr" => 133.322368,

    // Energy
    "J" => 1.0,
    "Wh" => 3600.0,
    "kWh" => 3_600_000.0,
    "cal" => 4.184,
    "kCal" => 4184.0,
    "kJ" => 1000.0,
    "eV" => 1.602176634e-19,
    "btu" => 1055.05585,
    "ftlb" => 1.35581795,
    "ustherm" => 1.055e8,

    // Force
    "N" => 1.0,
    "dyn" => 1e-5,
    "kp" => 9.80665,
    "pdl" => 0.138255,

    // Frequency
    "Hz" => 1.0,
    "kHz" => 1e3,
    "MHz" => 1e6,
    "GHz" => 1e9,

    // Data Storage
    "b" => 1.0,
    "k" => 1e3,
    "Mb" => 1e6,
    "Gb" => 1e9,
    "Tb" => 1e12,
    "KB" => 8.0 * 1e3,
    "MB" => 8.0 * 1e6,
    "GB" => 8.0 * 1e9,
    "TB" => 8.0 * 1e12,
    "kibibit" => 1024.0,
    "mebibit" => 1024.0 * 1024.0,

    // Data Transfer Rate
    "bps" => 1.0,
    "kbps" => 1e3,
    "Mbps" => 1e6,
    "Gbps" => 1e9,
    "Tbps" => 1e12,
    "kBps" => 8.0 * 1e3,
    "MBps" => 8.0 * 1e6,
    "GBps" => 8.0 * 1e9,
    "TBps" => 8.0 * 1e12,
    "Kibitps" => 1024.0,
    "Mibit" => 1024.0 * 1024.0,

    // Fuel Economy
    "kml" => 1.0,
    "l100km" => 100.0,
    "mpg" => 0.354006,
    "usmpg" => 0.425144,

    // Luminous Energy
    "lms" => 1.0,
    "lmh" => 3600.0,
    "lmmin" => 60.0,
    "T" => 1.0,

    // Magnetomotive Force
    "AT" => 1.0,
    "Gi" => 0.7957747,

    // Power
    "W" => 1.0,
    "hp" => 745.699872,
    "ps" => 735.49875,

    // Time
    "second" => 1.0,
    "millisecond" => 1e-3,
    "microsecond" => 1e-6,
    "nanosecond" => 1e-9,
    "minute" => 60.0,
    "hour" => 3600.0,
    "day" => 86400.0,
    "week" => 604800.0,
    "month" => 2.628e6,
    "year" => 3.154e7,
    "decade" => 3.154e8,
    "century" => 3.154e9,
};
