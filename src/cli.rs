use clap::{Arg, Command};

#[derive(Eq, PartialEq)]
pub enum Getter {
    Appliances,
    ApiKeys,
    Bundles,
    Customizations,
    CustomDevices,
    Dashboards,
    Devices,
    DeviceGroups,
    Extrahop,
    Licenses,
    Nodes,
    Networks,
    NetworkLocalities,
    PacketCaptures,
    RunningConfig,
    Software,
    Tags,
    ThreatCollections,
    Triggers,
    Vlans,
    None,
}

pub struct Cli {
    pub backup: bool,
    pub backup_device: String,
    pub getter: Getter,
}

impl Cli {
    fn default() -> Self {
        Self {
            backup: false,
            backup_device: String::from(""),
            getter: Getter::None,
        }
    }
    pub fn new() -> Self {
        let matches = Command::new("ehctl")
            .version("0.1.6")
            .about("Extrahop CLI")
            .subcommand(
                Command::new("backup")
                    .about("backup device customizations (currently `all` devices are backed up)")
                    .arg(
                        Arg::new("device")
                            .help("`all` or `device name` to backup")
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("get").about("get <endpoint>").arg(
                    Arg::new("endpoint")
                        .help("the uri endpoint to get")
                        .required(true),
                ),
            )
            .get_matches();

        let mut cli = Cli::default();

        // backup
        if let Some(backup_matches) = matches.subcommand_matches("backup") {
            if let Some(device) = backup_matches.value_of("device") {
                // println!("backup device: {device}");
                if device == "all" {
                    cli.backup = true;
                    cli.backup_device = String::from(device)
                } else {
                    println!("=> unknown device `{device}`");
                    cli.backup = false
                }
            };
        }
        // get
        else if let Some(get_matches) = matches.subcommand_matches("get") {
            if let Some(getter) = get_matches.value_of("endpoint") {
                cli.getter = match getter {
                    "apikeys" => Getter::ApiKeys,
                    "appliances" => Getter::Appliances,
                    "bundles" => Getter::Bundles,
                    "customizations" => Getter::Customizations,
                    "customdevices" => Getter::CustomDevices,
                    "dashboards" => Getter::Dashboards,
                    "devicegroups" => Getter::DeviceGroups,
                    "devices" => Getter::Devices,
                    "extrahop" => Getter::Extrahop,
                    "licenses" => Getter::Licenses,
                    "networks" => Getter::Networks,
                    "networklocalities" => Getter::NetworkLocalities,
                    "nodes" => Getter::Nodes,
                    "packetcaptures" => Getter::PacketCaptures,
                    "runningconfig" => Getter::RunningConfig,
                    "software" => Getter::Software,
                    "tags" => Getter::Tags,
                    "threatcollections" => Getter::ThreatCollections,
                    "triggers" => Getter::Triggers,
                    "vlans" => Getter::Vlans,
                    _ => {
                        println!("=> unknown endpoint `{}`", getter);
                        Getter::None
                    }
                };
            }
        }
        cli
    }
}
