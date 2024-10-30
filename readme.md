An efficient Thumbnailer for Canon CR2 and CR3 files by utilizing the included Thumbnails. No conversion needed.
(currently only tested working for nautilus on Ubuntu and Fedora Workstation)

# Installation
## Using the .deb Package
Download the .deb Package from the releases tab on the right. Then open a terminal in your Downloads folder. This can be done by right-clicking in your File Manager and clicking on `Open in Terminal` or by opening a Terminal and entering
```Shell
cd $HOME/Downloads/
```
To install the package, enter:
```Shell
sudo dpkg -i craw_thumbnailer.deb
```
OR
```Shell
sudo apt install ./craw_thumbnailer.deb
```
## From source
First, make sure you have rustc and cargo installed.
Then clone the repository and move into it:
```Shell
git clone https://github.com/Frittierapparat/craw_thumbnailer.git
cd craw_thumbnailer
```
Build the executable:
```Shell
cargo build --release
```
Copy the executable to the `/usr/bin/` directory:
```Shell
sudo cp target/release/craw_thumbnailer /usr/bin/
```
Next, add a thumbnailer entry at `/usr/share/thumbnailers`:
```Shell
sudo touch /usr/share/thumbnailers/craw_thumbnailer.thumbnailer
```
And add the following contents:
```desktop
[Thumbnailer Entry]
TryExec=/usr/bin/craw_thumbnailer
Exec=/usr/bin/craw_thumbnailer -f %i -o %o -s %s
MimeType=image/x-canon-cr2;image/x-canon-cr3
```
Finally, restart your file manager. (For example `nautilus -q`)
You may need to reset your Thumbnail Cache. To do that, delete everything inside `~/.cache/thumbnails`:
```Shell
cd ~/.cache/thumbnails/
rm -r *
```

# Basic Usage
The program currently takes 3 arguments:
```
-f/--file: the input file
-o/--output: the output file
-s/--size: the width in pixels you want (leave empty for original size)
-h/--help: display a basic help screen
```
