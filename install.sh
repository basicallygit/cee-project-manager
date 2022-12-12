#!/usr/bin/sh
if ! command -v cargo &> /dev/null
then
    echo "cargo not found. install rustup before building"
    exit
fi
cargo build --release
echo "add cee binary to /usr/bin? (y/N)"
read choice

case $PREFIX in
    *termux*)
        BINPATH="$PREFIX/bin/cee";
        SUDOCMD="";;
    *)
        BINPATH="/usr/bin/cee";
        SUDOCMD="sudo";;
esac

case $choice in 
    [yY])
        if [ -e $BINPATH ]
        then
            echo "$BINPATH exists, overwrite? (y/N)"
            read overwrite
            case $overwrite in 
                [yY])
                    eval "$SUDOCMD cp target/release/cee $BINPATH"
                    exit;;
                *)
                    exit;;
            esac
        else
            eval "$SUDOCMD cp target/release/cee $BINPATH"
        fi
        exit;;
    *)
        exit;;
esac
