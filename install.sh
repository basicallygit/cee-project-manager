if ! command -v cargo &> /dev/null
then
    echo "cargo not found. install rustup before building"
    exit
fi
cargo build --release
echo "add cee binary to /usr/bin? (y/N)"
read choice
#check if on termux - path for usr/bin is different
SUDONEEDED=1
if [[ "$PREFIX" == *com.termux* ]]
then
    BINPATH="${$PREFIX}/bin/cee"
    SUDONEEDED=0
else
    BINPATH="/usr/bin/cee"
fi
case $choice in 
    [yY])
        if [ -e $BINPATH ]
        then
            echo "/usr/bin/cee exists, overwrite? (y/N)"
            read overwrite
            case $overwrite in 
                [yY])
                    if [ $SUDONEEDED -eq 1 ]; then
                        sudo cp ./target/release/cee $BINPATH
                    else
                        cp ./target/release/cee $BINPATH
                    fi
                    exit;;
                *)
                    exit;;
            esac
        else
            if [ $SUDONEEDED -eq 1 ]; then
                sudo cp ./target/release/cee $BINPATH
            else
                cp ./target/release/cee $BINPATH
            fi
        fi
        exit;;
    *)
        exit;;
esac