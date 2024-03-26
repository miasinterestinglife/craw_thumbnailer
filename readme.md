An efficient Thumbnailer for Canon CR2 and CR3 files (CRW still in development) by utilizing the included Thumbnails. No conversion needed.

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
touch /usr/share/thumbnailers/craw_thumbnailer.thumbnailer
```
And add the following contents:
```desktop
[Thumbnailer Entry]
TryExec=/usr/bin/craw_thumbnailer
Exec=/usr/bin/craw_thumbnailer -f %i -o %o
MimeType=image/x-canon-cr2;image/x-canon-cr3
```
Finally, restart your file manager. (For example `nautilus -q`)
