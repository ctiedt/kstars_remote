use std::error::Error;

use dbus_codegen::GenOpts;

fn main() -> Result<(), Box<dyn Error>> {
    let intf_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/kstars_interfaces");
    if !std::fs::exists(intf_dir)? {
        eprintln!(
            "Interface XMLs have not been generated yet. Please run ./gen_dbus_interfaces.sh first."
        );
        std::process::exit(-1);
    }
    for interface in std::fs::read_dir(intf_dir)? {
        let path = interface?.path();
        let xmldata = std::fs::read_to_string(&path)?;
        let opts = GenOpts {
            crossroads: false,
            methodtype: None,
            ..Default::default()
        };
        let generated = dbus_codegen::generate(&xmldata, &opts)?;
        std::fs::write(
            format!(
                "{}/{}.rs",
                std::env::var("OUT_DIR")?,
                path.file_stem().expect("trust me bro").to_string_lossy(),
            ),
            generated,
        )?;
    }
    Ok(())
}
