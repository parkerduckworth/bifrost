use bifrost::ops::bifrost_run;
use bifrost::core::workspace;
use bifrost::util::BifrostResult;

pub fn exec(ws: &workspace::WorkSpace) -> BifrostResult<()> {
    return bifrost_run::run(ws);
}
