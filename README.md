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

![image-20231013222653263](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013222653263.jpg)

`sudo mv hina /usr/bin`

Move Hina to the bin file directory, then you can type hina in everywhere.



## Execution Flags

`-r` for **recursively** run the command if supported. **Default false.**

`-h` for translate the output to **human-readable**. e.g. `10240 KB` will be translate to `10 MB`. **Default false.**

`-i` for **input** if the command can receive an input parameter. **Default empty string**

`-o` for **output** if the command can receive an output parameter. **Default empty string**

`-a` for **append** if the command can receive an append parameter. **Default empty string**

`-n` for **num** if the command can receive a num parameter. **Default 0**



## Process Functions

### Detailed process memory usage

**Support flags: `-h`**

Although htop or top can easily check how much memory is used by each process and the usage of swap, if you want to check how much swap is used by each process, they are unable to do anything.

Hina provides the ability to view **detailed memory usage** of each process, including **Size, Swap, RSS, PSS** and other information, and can be sorted by specified fields.

![image-20231013213858318](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013213858318.jpg)

The `-h` parameter can make the usage **human-readable**, and swap specifies the sorting field as swap, sorting from low to high.

![image-20231013213928815](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013213928815.jpg)



### Process ancestors

Use "pa" to list a process's all ancestors.

![image-20231020223912023](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231020223912023.jpg)



### Process filtering

This function is similar to ps, but only lists the processes belonging to this user. Compared with `ps -ef | grep $USER`, it can filter out processes run by other users, but the command contains `$USER`, such as other users running some of the programs and it includes some files in your directory. These programs will be listed by `ps -ef | grep $USER`, but `hina ps` will only list processes created by `$USER`.

`ps -ef | grep $USER` listed processes that are not created by the user:

![ps lists processes not created by $USER](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013214122143.jpg)

`hina ps ` will filter out those:

![hina filtered ps](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013214155554.jpg)





## Recycle bin

**Give your rm a chance to recover**.

A recycle bin system managed by Hina, using `rm` to delete files or directories and `rs` to restore them.

**Note**: These files are actually moved to `$HOME/.hina/RecycleBin`, so remember to use `hina et` to clean up the Recycle Bin every once in a while to avoid taking up your space.

**Remove**:

![image-20231013213101438](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013213101438.jpg)

**Restore**:

![image-20231013213136108](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013213136108.jpg)

Hina will distinguish deleted files or directories by **adding a 16-digit random string**, so there is no need to worry about conflicts caused by deleting two files with the same name.

![image-20231013213351691](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013213351691.jpg)

Use `hina et` to empty the recycle bin.

![image-20231013214446956](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013214446956.jpg)



## Filesystem Functions

### Symlink & Hardlink conversion

Hina provides **conversion** between **symbolic links** and **hard links**.

#### Symlink to Hardlink

**Support flags: `-r`**

![image-20231013214819810](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013214819810.jpg)

Hina will **recursively** traverse the folder, resolve all symbolic links, hard-link the source files to the target, and **delete expired symbolic links.**

![image-20231013214853699](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013214853699.jpg)



#### Hardlink to Symlink

**Support flags: `-r`**

For this reverse operation, **parsing the hard-linked source file requires querying the file with the same inode through the inode, so the l2s command requires a source file root directory** (this directory can be left blank, which means searching for the source file from the root directory `/`, But it will be very time-consuming).

![image-20231013215121225](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231013215121225.jpg)



### Make nested directory

**Support flags: `-r`**

`mkndir` will generate the directory for each file using its filename without extension

![image-20231022223118828](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231022223118828.jpg)

Here is a sample that some dirs have some file, then execute `hina mkndir -r` with recursive option.

![image-20231022223212585](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231022223212585.jpg)



### Batch Renaming

**Support flags: `-r,-i,-o,-a,-n`**

Use `rn` to perform batch renaming if you have a large number of files that need to be renamed in the same pattern.

Now we have some files TEST.txt, but the strange string abc appears in different places in the file name.

![image-20231022225118647](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231022225118647.jpg)

With Hina renaming function, we can easily remove the "abc" from the filename.

![image-20231022225214669](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231022225214669.jpg)

With the `-a` parameter, we can add some pattern in the filename, such as adding "HINA" to every file.

![image-20231022225430691](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231022225430691.jpg)

Also, the `-n` parameters allows adding pattern to specific position, such as this time we can add "HINA" after "TEST".

![image-20231022225522741](https://raw.githubusercontent.com/B1ACK917/img_asset/main/image-20231022225522741.jpg)
