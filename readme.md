# Bright - Rust Backlight Controller for Linux

## Usage
Bright is a Rust program designed to help you control the backlight brightness on your Linux system.
Please ensure that your backlight is exposed in userspace at the /sys/backlight/ directory.

To adjust the brightness, follow these steps:
1. Make sure you have superuser privileges. You can use the sudo command to run Bright with elevated permissions.
2. Open your terminal and use the following command, replacing <BRIGHTNESS_SCALE> with your desired brightness level:

```sh
sudo bright <BRIGHTNESS_SCALE>
```

BRIGHTNESS_SCALE is a value between 0 and 1, allowing you to set the brightness level anywhere from black to maximum brightness.
Please keep in mind that this is my first attempt at creating a Rust program, so it may have some room for improvement. Your feedback and contributions are welcome!