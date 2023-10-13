# Hina

**Hina** is a multi-purpose Linux command line tool, including **process filtering**, viewing **details of process memory usage** (SWAP, RSS, PSS...), **file system-related** operations (soft and hard link conversion...) and other purposes.

I wrote this tool out of personal hobby and real needs to facilitate my management of my Linux server.



## How to install Hina

### Install Rust

Hina is written in the Rust language, so you need a Rust compilation environment.

Open a terminal and enter the following command:

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

This command will download a script and start installing the rustup tool, which will install the latest stable version of Rust. You may be prompted for your administrator password.

If the installation is successful, the following line will appear:

`Rust is installed now. Great!`

OK, this completes the Rust installation.



### Build Hina

Open a terminal and enter the following command:

`git clone https://github.com/B1ACK917/Hina.git`

This will clone the git repo to your machine.

`cd Hina && sh build.sh`

Then the build script will running and compile the Hina binary file, you can find the file in current directory, which named `hina`.

![image-20231013222653263](./asset/image-20231013222653263.jpg)

`sudo mv hina /usr/bin`

Move Hina to the bin file directory, then you can type hina in everywhere.



## Detailed process memory usage

Although htop or top can easily check how much memory is used by each process and the usage of swap, if you want to check how much swap is used by each process, they are unable to do anything.

Hina provides the ability to view **detailed memory usage** of each process, including **Size, Swap, RSS, PSS** and other information, and can be sorted by specified fields.

![image-20231013213858318](./asset/image-20231013213858318.jpg)

The `-h` parameter can make the usage **human-readable**, and swap specifies the sorting field as swap, sorting from low to high.

![image-20231013213928815](./asset/image-20231013213928815.jpg)



## Recycle bin

**Give your rm a chance to recover**.

A recycle bin system managed by Hina, using `rm` to delete files or directories and `rs` to restore them.

**Note**: These files are actually moved to `$HOME/.hina/RecycleBin`, so remember to use `hina et` to clean up the Recycle Bin every once in a while to avoid taking up your space.

**Remove**:

![image-20231013213101438](./asset/image-20231013213101438.jpg)

**Restore**:

![image-20231013213136108](./asset/image-20231013213136108.jpg)

Hina will distinguish deleted files or directories by **adding a 16-digit random string**, so there is no need to worry about conflicts caused by deleting two files with the same name.

![image-20231013213351691](./asset/image-20231013213351691.jpg)

Use `hina et` to empty the recycle bin.

![image-20231013214446956](./asset/image-20231013214446956.jpg)



## Process filtering

This function is similar to ps, but only lists the processes belonging to this user. Compared with `ps -ef | grep $USER`, it can filter out processes run by other users, but the command contains `$USER`, such as other users running some of the programs and it includes some files in your directory. These programs will be listed by `ps -ef | grep $USER`, but `hina ps` will only list processes created by `$USER`.



`ps -ef | grep $USER` listed processes that are not created by the user:

![ps lists processes not created by $USER](./asset/image-20231013214122143.jpg)

`hina ps ` will filter out those:

![hina filtered ps](./asset/image-20231013214155554.jpg)



## Symlink & Hardlink conversion

Hina provides **conversion** between **symbolic links** and **hard links**.

### Symlink to Hardlink

![image-20231013214819810](./asset/image-20231013214819810.jpg)

Hina will **recursively** traverse the folder, resolve all symbolic links, hard-link the source files to the target, and **delete expired symbolic links.**

![image-20231013214853699](./asset/image-20231013214853699.jpg)



## Hardlink to Symlink

For this reverse operation, **parsing the hard-linked source file requires querying the file with the same inode through the inode, so the l2s command requires a source file root directory** (this directory can be left blank, which means searching for the source file from the root directory `/`, But it will be very time-consuming).

![image-20231013215121225](./asset/image-20231013215121225.jpg)