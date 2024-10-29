sudo ifconfig tun0 down
sudo ip link set tun0 down
sudo ip link delete tun0
sudo ip tuntap del mode tap tun0
CARGO_TARGET_DIR=./target
echo "Target Directory: $CARGO_TARGET_DIR"
# sudo rm -rf target # Uncomment this for a fresh start
cargo build --release
sudo setcap cap_net_admin+ep $CARGO_TARGET_DIR/release/tun_cp
$CARGO_TARGET_DIR/release/tun_cp & 
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid
cls