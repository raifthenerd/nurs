use std::num::ParseIntError;
use std::str::FromStr;

use clap::Args;
use serde::{de, ser, Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum Memory {
    KB(u16),
    MB(u16),
    GB(u16),
}
impl ToString for Memory {
    fn to_string(&self) -> String {
        match self {
            Memory::KB(amount) => format!("{}k", amount),
            Memory::MB(amount) => format!("{}m", amount),
            Memory::GB(amount) => format!("{}g", amount),
        }
    }
}
impl FromStr for Memory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf: String = s.to_owned();
        let unit = buf.pop().ok_or("cannot parse empty string")?;
        let unit = match unit {
            'k' | 'K' => Memory::KB,
            'm' | 'M' => Memory::MB,
            'g' | 'G' => Memory::GB,
            ch => return Err(format!("invalid memory unit: {}", ch)),
        };
        let amount = buf.parse().map_err(|why: ParseIntError| why.to_string())?;
        Ok(unit(amount))
    }
}
impl Serialize for Memory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for Memory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let raw: &str = Deserialize::deserialize(deserializer)?;
        raw.parse().map_err(de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize, Args)]
pub struct Profile {
    /// Override the number of cores
    #[clap(short, long, value_name = "##")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<u8>,
    /// Override the number of GPU
    #[clap(short, long, value_name = "#")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<u8>,
    /// Override the amount of memory
    #[clap(short, long, value_name = "##k/##m/##g")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem: Option<Memory>,
    /// Override the name of partition
    #[clap(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    /// Override the name of node
    #[clap(short, long, multiple_values = true)]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub nodes: Vec<String>,
    /// Override the command to execute on slurm
    #[clap(last = true, multiple_values = true)]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub command: Vec<String>,
}
impl Profile {
    pub fn overwrite(self, other: Profile) -> Profile {
        Profile {
            cpu: other.cpu.or(self.cpu),
            gpu: other.gpu.or(self.gpu),
            mem: other.mem.or(self.mem),
            partition: other.partition.or(self.partition),
            nodes: if !other.nodes.is_empty() {
                other.nodes
            } else {
                self.nodes
            },
            command: if !other.command.is_empty() {
                other.command
            } else {
                self.command
            },
        }
    }
}
