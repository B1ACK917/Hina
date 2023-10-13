# Hina

**Hina** is a multi-purpose Linux command line tool, including **process filtering**, viewing **details of process memory usage** (SWAP, RSS, PSS...), **file system-related** operations (soft and hard link conversion...) and other purposes.

I wrote this tool out of personal hobby and real needs to facilitate my management of my Linux server.



## Process filtering

This function is similar to ps, but only lists the processes belonging to this user. Compared with `ps -ef | grep $USER`, it can filter out processes run by other users, but the command contains `$USER`, such as other users running some of the programs and it includes some files in your directory. These programs will be listed by `ps -ef | grep $USER`, but `hina ps` will only list processes created by `$USER`.

![ps lists processes not created by $USER](./asset/image-20231013211220786.png)

![hina filtered ps](./asset/image-20231013211251936.png)



## Recycle bin

**Give your rm a chance to recover**.

A recycle bin system managed by Hina, using `rm` to delete files or directories and `rs` to restore them.

**Note**: These files are actually moved to `$HOME/.hina/RecycleBin`, so remember to use `hina et` to clean up the Recycle Bin every once in a while to avoid taking up your space.



