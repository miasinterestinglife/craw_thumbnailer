An efficient Thumbnailer for Canon CR2 and CR3 files (CRW still in development) by utilizing the included Thumbnails. No conversion needed.
(currently only tested working for nautilus on Ubuntu and Fedora Workstation)

# Installation
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

# Basic Usage
The program currently takes 3 arguments:
'''
-f/--file: the input file
-o/--output: the output file
-s/--size: the width in pixels you want (leave empty for original size)
-h/--help: display a basic help screen
'''