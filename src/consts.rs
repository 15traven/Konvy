use phf::phf_map;

pub const HELP_DOC_LINK: &str = "https://github.com/15traven/konvy";

pub static FACTORS: phf::Map<&'static str, (&'static str, f64)> = phf_map! {
    // Mass
    "kg" => ("mass", 1.0),
    "g" => ("mass", 0.001),
    "mg" => ("mass", 1e-6),
    "t" => ("mass", 1000.0),
    "oz" => ("mass", 0.0283495),
    "lb" => ("mass", 0.453592),
    "stone" => ("mass", 6.35029),
    "ct" => ("mass", 0.0002),

    // Length
    "m" => ("length", 1.0),
    "km" => ("length", 1000.0),
    "cm" => ("length", 0.01),
    "mm" => ("length", 0.001),
    "inch" => ("length", 0.0254),
    "ft" => ("length", 0.3048),
    "yard" => ("length", 0.9144),
    "mile" => ("length", 1609.344),
    "NM" => ("length", 1852.0),

    // Angle
    "rad" => ("angle", 1.0),
    "deg" => ("angle", std::f64::consts::PI / 180.0),
    "grad" => ("angle", std::f64::consts::PI / 200.0),
    "mrad" => ("angle", 0.001),
    "arcmin" => ("angle", std::f64::consts::PI / 10_800.0),
    "arcseconds" => ("angle", std::f64::consts::PI / 648_000.0),

    // Area
    "m2" => ("area", 1.0),
    "km2" => ("area", 1_000_000.0),
    "acre" => ("area", 4046.8564224),
    "hectare" => ("area", 10_000.0),
    "feet2" => ("area", 0.09290304),
    "inch2" => ("area", 0.00064516),
    "mile2" => ("area", 2_589_988.110336),
    "yard2" => ("area", 0.83612736),

    // Volume
    "l" => ("volume", 1.0),
    "ml" => ("volume", 0.001),
    "kl" => ("volume", 1000.0),
    "m3" => ("volume", 1000.0),
    "inch3" => ("volume", 0.0163871),
    "feet3" => ("volume", 28.3168),
    "pt" => ("volume", 0.568261),
    "qt" => ("volume", 1.13652),
    "gal" => ("volume", 4.54609),
    "uscup" => ("volume", 0.24),
    "usfloz" => ("volume", 0.0295735),
    "uspt" => ("volume", 0.473176),
    "usqt" => ("volume", 0.946353),
    "usgal" => ("volume", 3.78541),
    "ustbsp" => ("volume", 0.0147868),
    "ustsp" => ("volume", 0.00492892),
    "floz" => ("volume", 0.0284131),
    "tbsp" => ("volume", 0.0177582),
    "tsp" => ("volume", 0.00591939),
    "oilbarrel" => ("volume", 158.987),

    // Speed
    "mps" => ("speed", 1.0),
    "fps" => ("speed", 0.3048),
    "kph" => ("speed", 0.277778),
    "mph" => ("speed", 0.44704),
    "knot" => ("speed", 0.514444),

    // Pressure
    "Pa" => ("pressure", 1.0),
    "atm" => ("pressure", 101325.0),
    "bar" => ("pressure", 100000.0),
    "psi" => ("pressure", 6894.757),
    "Torr" => ("pressure", 133.322368),

    // Energy
    "J" => ("energy", 1.0),
    "Wh" => ("energy", 3600.0),
    "kWh" => ("energy", 3_600_000.0),
    "cal" => ("energy", 4.184),
    "kCal" => ("energy", 4184.0),
    "kJ" => ("energy", 1000.0),
    "eV" => ("energy", 1.602176634e-19),
    "btu" => ("energy", 1055.05585),
    "ftlb" => ("energy", 1.35581795),
    "ustherm" => ("energy", 1.055e8),

    // Force
    "N" => ("force", 1.0),
    "dyn" => ("force", 1e-5),
    "kp" => ("force", 9.80665),
    "pdl" => ("force", 0.138255),

    // Frequency
    "Hz" => ("frequency", 1.0),
    "kHz" => ("frequency", 1e3),
    "MHz" => ("frequency", 1e6),
    "GHz" => ("frequency", 1e9),

    // Data Storage
    "b" => ("data", 1.0),
    "k" => ("data", 1e3),
    "Mb" => ("data", 1e6),
    "Gb" => ("data", 1e9),
    "Tb" => ("data", 1e12),
    "KB" => ("data", 8.0 * 1e3),
    "MB" => ("data", 8.0 * 1e6),
    "GB" => ("data", 8.0 * 1e9),
    "TB" => ("data", 8.0 * 1e12),
    "kibibit" => ("data", 1024.0),
    "mebibit" => ("data", 1024.0 * 1024.0),

    // Data Transfer Rate
    "bps" => ("datarate", 1.0),
    "kbps" => ("datarate", 1e3),
    "Mbps" => ("datarate", 1e6),
    "Gbps" => ("datarate", 1e9),
    "Tbps" => ("datarate", 1e12),
    "kBps" => ("datarate", 8.0 * 1e3),
    "MBps" => ("datarate", 8.0 * 1e6),
    "GBps" => ("datarate", 8.0 * 1e9),
    "TBps" => ("datarate", 8.0 * 1e12),
    "Kibitps" => ("datarate", 1024.0),
    "Mibit" => ("datarate", 1024.0 * 1024.0),

    // Fuel Economy
    "kml" => ("fueleconomy", 1.0),
    "l100km" => ("fueleconomy", 100.0),
    "mpg" => ("fueleconomy", 0.354006),
    "usmpg" => ("fueleconomy", 0.425144),

    // Luminous Energy
    "lms" => ("luminous", 1.0),
    "lmh" => ("luminous", 3600.0),
    "lmmin" => ("luminous", 60.0),
    "T" => ("luminous", 1.0),

    // Magnetomotive Force
    "AT" => ("magneto", 1.0),
    "Gi" => ("magneto", 0.7957747),

    // Power
    "W" => ("power", 1.0),
    "hp" => ("power", 745.699872),
    "ps" => ("power", 735.49875),

    // Time
    "second" => ("time", 1.0),
    "millisecond" => ("time", 1e-3),
    "microsecond" => ("time", 1e-6),
    "nanosecond" => ("time", 1e-9),
    "minute" => ("time", 60.0),
    "hour" => ("time", 3600.0),
    "day" => ("time", 86400.0),
    "week" => ("time", 604800.0),
    "month" => ("time", 2.628e6),
    "year" => ("time", 3.154e7),
    "decade" => ("time", 3.154e8),
    "century" => ("time", 3.154e9),
};
