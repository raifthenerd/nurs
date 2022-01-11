# nurs

```bash
> nurs
nurs 0.1.0
nurs makes srun easy to use with predefined configurations

USAGE:
    nurs [OPTIONS] [-- <COMMAND>...]

ARGS:
    <COMMAND>...    Override the command to execute on slurm

OPTIONS:
        --config <PATH>            Custom config file location
        --profile <NAME>           Profile name to use
        --export <PATH>            Export to sbatch script instead of executing srun command
    -c, --cpu <##>                 Override the number of cores
    -g, --gpu <#>                  Override the number of GPU
    -m, --mem <##k/##m/##g>        Override the amount of memory
    -p, --partition <PARTITION>    Override the name of partition
    -n, --nodes <NODES>...         Override the name of node
        --dry-run                  DO NOT execute; just print the command/script on stdout
    -h, --help                     Print help information
    -V, --version                  Print version information
```

## Configuration

Configuration file looks like:

```toml
default = "train"      # name of default profile

[profile.train]        # profile section; all fields are optional
cpu = 12               # number of cores   (default = 1)
gpu = 1                # number of gpu     (default = 0)
mem = "2G"             # amount of memory  (default = 10m)
partition = "dev_gpu"  # name of partition

[profile.monitor]
nodes = ["dev101"]     # name of node
command = ["htop"]     # command to execute on slurm

[profile.tensorboard]
partition = "dev"
command = ["tensorboard", "--bind-all"]
```

`nurs` will detect the configuration file located in one of the following:

- `CUSTOM_PATH`, when `--config CUSTOM_PATH` argument is given
- `$PWD/.nurs.toml`, `$PWD/../.nurs.toml`, `$PWD/../../.nurs.toml` and so on...
- `$XDG_CONFIG_DIR/nurs.toml` or `$HOME/.config/nurs.toml`
- `$HOME/.nurs.toml`

## Usage

Remove `--dry-run` to actually execute the script/command.

```bash
> nurs --dry-run -- python script.py
srun --partition=dev_gpu --nodes=1 --cpus-per-task=12 --mem=2g --gres=gpu:1 --pty python script.py

> nurs --dry-run --export sbatch.sh -- python script.py
#!/bin/bash

#SBATCH --partition=dev_gpu
#SBATCH --nodes=1
#SBATCH --cpus-per-task=12
#SBATCH --mem=2g
#SBATCH --gres=gpu:1

python script.py

> nurs --dry-run --profile monitor
srun --nodelist=dev101 --nodes=1 --cpus-per-task=1 --mem=10m --gres=gpu:0 --pty htop

> nurs --dry-run --profile monitor --mem 2m
srun --nodelist=dev101 --nodes=1 --cpus-per-task=1 --mem=2m --gres=gpu:0 --pty htop

> nurs --dry-run --profile monitor -- top
srun --nodelist=dev101 --nodes=1 --cpus-per-task=1 --mem=10m --gres=gpu:0 --pty top
```
