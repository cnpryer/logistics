# general
apt-get update -y
apt-get upgrade -y
apt-get dist-upgrade -y
apt-get install build-essential -y
apt-get install -y vim curl

# rust
echo 'curl https://sh.rustup.rs -sSf | sh -s -- -y;' | su vagrant