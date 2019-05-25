# Bifrost Journal

#### Friday May 24, 2019 
> @parkerduckworth

Docker containers exit after the main process in the container has finished
executing.  In our case, this means once the first bash command has executed
to completion, the container shuts down.

The problem with this of course, is that Bifrost users will likely want to
run more than one command at a time.  

I thought for a while about the best approach to take with this in mind.

##### Possible Approaches
One possible route is to generate an entrypoint shell script based on the
commands in `Bifrost.toml` that calls and tracks each command, running until 
completion or a nonzero exit status.

The problem with this approach is that generated code is difficult to 
validate at compile-time (not to mention bash is well, bash).

Another approach is to create a new Docker container that is never cleaned
up with `--rm`.  The container can be restarted in a loop, calling
`docker start bash -c "<commands>"` each iteration.  However this is not an
efficient way to do things.

##### Selected Solution
I tried both approaches above, and several others.  After much planning, testing,
and head-scratching, I chose to run each command as one string in one container 
instance, in sequence, joined by `&&`, so that all execution stops at the first 
failure. The container instance runs the joined command string until completion, 
or until a command fails. 

This implementation is the least sexy, but the most efficient and robust. 
Although all commands are called in sequence, each command's output and error 
streams are captured separately, and are aggregated in the top-level 
`bifrost_rin::run` function.

##### Improvements / Unexplored Options
Another thing I toyed around with was if docker container instance was not 
able to find a command, Bifrost would automatically pull the respective 
docker image, or write to the current `Dockerfile` and rebuild the existing
one. Unfortunately, command names do not always match up with their package
names, and just guessing the package name does not seem like a good method.

The current execution of `bifrost run` is robust, easy to debug, and gets
the job done.  However, its robustness is due to its simplicity, and its
simplicity may one day hinder a future feature.  

I've been thinking a lot about threads, and how we may be able to spawn a 
thread for each command (or let the user specify whether certain commands 
ought to be ran together).  This would allow each command (or command sequence)
to be executed and inspected individually. I'm not sure if this would work, or if
Docker would allow threads to be blocked in order to run all commands in the same
instance. Keeping this in my back pocket.

#### Thursday May 2, 2019
> @ericdeansanchez

I think it still makes sense to have a global container. However, I don't think
it makes sense to have a global config file. Now that there are workspaces,
users can initialize a `WorkSpace` in their current directory. In this current
working directory it makes sense to have a local config file––much like a `.git`
file.

If this change is made then there will need to be safeguards to prevent 
performing bifrost operations in non-bifrost-initialized directories.

I think `utils.rs` needs some work.

##### Concerns: Owners and Borrowers

###### ./src/config.rs

* ~~fn parse(path: &Option<PathBuf>, config_map: &mut HashMap<String
bool>)~~

###### ./src/utils.rs

**[OK: WorkingDir should own its fields]**
* 18:    `pub root: PathBuf`
* 19:    `pub paths: Vec<PathBuf>`
* 20:    `pub ignore_list: HashSet<PathBuf>`

```rust
pub struct WorkingDir {
    pub root: PathBuf,
    pub paths: Vec<PathBuf>,
    pub ignore_list: HashSet<PathBuf>,
}
```

**[May warrant refactoring]**

* 24:    `pub fn init(cwd: Option<PathBuf>, ignore_list: Vec<PathBuf>)`
* 51:    `fn stash(path: PathBuf, content: &mut WorkingDir)`
* 57:    `fn ignore(&mut self, paths: Vec<PathBuf>)`
* 67:    `pub fn walk_dirs(dir: &PathBuf, visit: &VisitorMut, content: &mut`
* 87:    `pub fn path_builder(prefix: Option<PathBuf>, suffix: &str) -> 
Option<PathBuf>`