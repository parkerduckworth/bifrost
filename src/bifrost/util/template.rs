// Main application template.
pub const APP_TEMPLATE: &str = "

    ____  _ ____                __
   / __ )(_) __/_____*_*  _____/ /_
  / __  / / /_/ ___/ __ \\/ ___/ __/
 / /_/ / / __/ /  / /_/ (__  ) /_
/_____/_/_/ /_/   \\____/____/\\__/



AUTHORS:
{author}

ABOUT:
{about}

{all-args}

USAGE:{usage}
";

pub const BIFROST_USAGE: &str = "
    bifrost [COMMAND] [OPTION] [PROJECT]
    bifrost [COMMAND] [DIR | FILES | FILE]";

// Template for subcommand help messages. This is displayed when invoking
// `bifrost [SUBCOMMAND] --help` directly and in the event there is an
// error with respect to usage.
pub const SUBCOMMAND_HELP_TEMPLATE: &str = "
DESCRIPTION:
{about}

{all-args}

USAGE:{usage}
";

pub const EXPLICIT_LONG_HELP: &str = "

    ____  _ ____                __
   / __ )(_) __/_____*_*  _____/ /_
  / __  / / /_/ ___/ __ \\/ ___/ __/
 / /_/ / / __/ /  / /_/ (__  ) /_
/_____/_/_/ /_/   \\____/____/\\__/



AUTHORS:
ericdeansanchez <ericdeansanchez@berkeley.edu>,
parkerduckworth <parker_duckworth@icloud.com>

ABOUT:
Bridging the tool gap

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    init      Initialize a bifrost directory within the cwd
    load      Load directory, file, or files into the bifrost container
    run       Run command string(s) on a bifrost workspace
    show      Display files currently in the bifrost container
    unload    Unload a workspace from the bifrost container

USAGE:
    bifrost [COMMAND] [OPTION] [PROJECT]
    bifrost [COMMAND] [DIR | FILES | FILE]
";
