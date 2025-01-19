
# USB to TTL serial driver for MacOS 

The required driver for this board is `CH9102F` but the driver in 
`../usb2ttl_driver/CH341SER_MAC.ZIP` also includes this chip model.

Unzip the driver and open `CH34xVCPDriver.dmg` file 
Drag and drop the file to `Application` like any other app.
It will ask to give permission, this might change from on OS version to another,
so check the PDF in the zip file.

* Source:

https://github.com/WCHSoftGroup/ch34xser_macos

* Forum refrences:

https://arduino.stackexchange.com/a/86787

## Select port

Check under `/dev/*` if the driver was installed.

```sh
lla /dev | grep usb

crw-rw-rw-   1 root   wheel      0x9000003 Dec 31 16:31 cu.usbserial-58AB0097191
crw-rw-rw-   1 root   wheel      0x9000005 Dec 31 16:31 cu.wchusbserial58AB0097191
crw-rw-rw-   1 root   wheel      0x9000002 Dec 31 16:31 tty.usbserial-58AB0097191
crw-rw-rw-   1 root   wheel      0x9000004 Dec 31 16:31 tty.wchusbserial58AB0097191
```

Try selecting `cu.wchusbserial58AB0097191` in Arduino IDE under Tools > Port.

If the device doesn’t respond, try cu.usbserial-58AB0097191.
Both should work identically.

## Troubleshooting

### As driver extensions (dexts) MacOS Sequoia 15.2

Since macOS no longer allows third-party kernel extensions (kexts) easily, 
driver developers have to use Driver Extensions (dexts). 
These require user approval and special permission settings.

Check if the driver is installed as extension blocked

```sh
systemextensionsctl list

1 extension(s)
--- com.apple.system_extension.driver_extension 
---(Go to 'System Settings > General > Login Items & Extensions > Driver Extensions' to modify these system extension(s))
enabled active  teamID  bundleID (version)      name    [state]
*       *       5JZGQTGU4W      cn.wch.CH34xVCPDriver (1.0/1)   cn.wch.CH34xVCPDriver    [activated enabled]
```

see `System Settings > General > Login Items & Extensions > Driver Extensions`

Check if the driver is loaded

```sh 
systemextensionsctl list | grep -i ch34
*       *       5JZGQTGU4W      cn.wch.CH34xVCPDriver (1.0/1)   cn.wch.CH34xVCPDriver   [activated enabled]
```
At this point the driver might be activated but not shown in /dev/ 

` ls -l /dev/tty.* /dev/cu.*`

In this case try to change the usb cable. Before I just used a regular USB-C cable
to connect directly to ESP-32 UBS port, now I need a converter in between.

`ESP-32 (USB-C) -> USB-A -> USB-C -> MacBook`

See the comments on this issue.

https://github.com/espressif/arduino-esp32/issues/1084

### As Kernel extensions (kexts) Previous MacOS versions

verify if the driver is installed

`ls -l /Library/Extensions/ | grep -i ch34`

or 

`ls -l /System/Library/Extensions/ | grep -i ch34`

Check if the driver is loaded

`kextstat` was replaced by `kmutil` in the latest version of MacOS (Sequoia 15.2)

`kmutil | grep ch34x`

list all the loaded drivers, add grep if needed 

`kmutil showloaded`


