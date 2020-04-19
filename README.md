# CHART FOR TRADING

## Usage
```
usr/chart
```

## Compilation
```bash
./compile ARG1 ARG2
```

ARG1:
<pre>
<b>linux</b> windows
</pre>

ARG2:
<pre>
<b>debug</b> release
</pre>

## Structure & Ideas

- sh script `compile` for programmers to build the binaries (since there is no programmers for this project in Windows)
- `chart/usr` is the folder delivered to users
- sh & bat script `install` in `chart/usr` to check if SDL2 is accessible
  - Linux: check in command line if installed, if not install it
  - Windows: 
    - give dlls in `chart/usr/dll` and add the directory to the path
    - give dlls in `chart/usr/dll`, add the binary to the directory and create a shortcut in `chart/usr`
  - hide the binary to force the user to execute the `install` script
