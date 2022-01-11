use std::fmt::{self, Write};

use crate::profile::{Memory, Profile};

pub fn sbatch(profile: &Profile) -> Result<String, fmt::Error> {
    let mut script = String::new();
    writeln!(&mut script, "#!/bin/bash")?;
    writeln!(&mut script)?;

    if let Some(partition) = profile.partition.as_ref() {
        writeln!(&mut script, "#SBATCH --partition={}", partition)?;
    }
    if !profile.nodes.is_empty() {
        writeln!(&mut script, "#SBATCH --nodelist={}", profile.nodes.join(","))?;
    }

    writeln!(&mut script, "#SBATCH --nodes=1")?;
    writeln!(
        &mut script,
        "#SBATCH --cpus-per-task={}",
        profile.cpu.unwrap_or(1).max(1)
    )?;
    writeln!(
        &mut script,
        "#SBATCH --mem={}",
        profile.mem.unwrap_or(Memory::MB(10)).to_string()
    )?;
    writeln!(&mut script, "#SBATCH --gres=gpu:{}", profile.gpu.unwrap_or(0))?;

    if !profile.command.is_empty() {
        writeln!(&mut script)?;
        writeln!(&mut script, "{}", profile.command.join(" "))?;
    }
    Ok(script)
}

pub fn srun(profile: &Profile) -> Result<Vec<String>, fmt::Error> {
    let mut arguments = vec![];

    if let Some(partition) = profile.partition.as_ref() {
        arguments.push(format!("--partition={}", partition))
    }
    if !profile.nodes.is_empty() {
        arguments.push(format!("--nodelist={}", profile.nodes.join(",")))
    }

    arguments.push("--nodes=1".to_owned());
    arguments.push(format!("--cpus-per-task={}", profile.cpu.unwrap_or(1).max(1)));
    arguments.push(format!("--mem={}", profile.mem.unwrap_or(Memory::MB(10)).to_string()));
    arguments.push(format!("--gres=gpu:{}", profile.gpu.unwrap_or(0)));

    arguments.push("--pty".to_owned());
    if !profile.command.is_empty() {
        arguments.append(&mut profile.command.clone());
        Ok(arguments)
    } else {
        Err(fmt::Error)
    }
}
