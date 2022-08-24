if ! command -v cargo &> /dev/null
then
    echo "cargo not found. install rustup before building"
    exit
fi
cargo build --release
echo "add cee binary to /usr/bin? (y/N)"
read choice
case $choice in 
    [yY])
        if [ -e /usr/bin/cee ]
        then
            echo "/usr/bin/cee exists, overwrite? (y/N)"
            read overwrite
            case $overwrite in 
                [yY])
                    sudo cp ./target/release/cee /usr/bin/cee
                    exit;;
                *)
                    exit;;
            esac
        else
            sudo cp ./target/release/cee /usr/bin/cee
        fi
        exit;;
    *)
        exit;;
esac