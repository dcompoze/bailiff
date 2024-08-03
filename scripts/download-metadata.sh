#!/usr/bin/zsh

METADATA_DIR="$( dirname $( dirname $(realpath $0)))/metadata"

if [[ ! -d $METADATA_DIR ]]; then
    echo "metadata directory does not exist at: $METADATA_DIR"
    exit 1
fi

subxt metadata --format=bytes --url="wss://rpc.polkadot.io:443" > $METADATA_DIR/polkadot.scale
echo "metadata: polkadot"
subxt metadata --format=bytes --url="wss://kusama-rpc.polkadot.io:443" > $METADATA_DIR/kusama.scale
echo "metadata: kusama"
subxt metadata --format=bytes --url="wss://rpc.astar.network:443" > $METADATA_DIR/astar.scale
echo "metadata: astar"
subxt metadata --format=bytes --url="wss://rpc.shiden.astar.network:443" > $METADATA_DIR/shiden.scale
echo "metadata: shiden"
subxt metadata --format=bytes --url="wss://moonbeam.public.blastapi.io:443" > $METADATA_DIR/moonbeam.scale
echo "metadata: moonbeam"
subxt metadata --format=bytes --url="wss://moonriver.public.blastapi.io:443" > $METADATA_DIR/moonriver.scale
echo "metadata: moonriver"
subxt metadata --format=bytes --url="wss://acala-polkadot.api.onfinality.io:443/public-ws" > $METADATA_DIR/acala.scale
echo "metadata: acala"
subxt metadata --format=bytes --url="wss://karura.api.onfinality.io:443/public-ws" > $METADATA_DIR/karura.scale
echo "metadata: karura"


