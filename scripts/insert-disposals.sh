#!/usr/bin/zsh

set -e

echo -n "DATE_TIME_ZONE: "
read DATE_TIME_ZONE
echo -n "BASE_ASSET_NAME: "
read BASE_ASSET_NAME
echo -n "BASE_ASSET_AMOUNT: "
read BASE_ASSET_AMOUNT
echo -n "QUOTE_ASSET_NAME: "
read QUOTE_ASSET_NAME
echo -n "QUOTE_ASSET_AMOUNT: "
read QUOTE_ASSET_AMOUNT
echo -n "QUOTE_POUND_VALUE: "
read QUOTE_POUND_VALUE
echo -n "EXCHANGE_NAME: "
read EXCHANGE_NAME
echo -n "EXCHANGE_TRANSACTION_ID: "
read EXCHANGE_TRANSACTION_ID

if [[ $EXCHANGE_TRANSACTION_ID != NULL ]]; then
    EXCHANGE_TRANSACTION_ID="'$EXCHANGE_TRANSACTION_ID'"
fi

echo "INSERT: $DATE_TIME_ZONE, $BASE_ASSET_NAME, $BASE_ASSET_AMOUNT, $QUOTE_ASSET_NAME, $QUOTE_ASSET_AMOUNT, $QUOTE_POUND_VALUE, $EXCHANGE_NAME, $EXCHANGE_TRANSACTION_ID"
echo -n "[y/N]: "
read ANSWER
if [[ $ANSWER == 'y' ]]; then
    psql -h server.lan -p 5432 -U admin -d taxes --command="INSERT INTO disposals(date_time_zone,base_asset_name,base_asset_amount,quote_asset_name,quote_asset_amount,quote_pound_value,exchange_name,exchange_transaction_id) VALUES ('$DATE_TIME_ZONE','$BASE_ASSET_NAME',$BASE_ASSET_AMOUNT,'$QUOTE_ASSET_NAME',$QUOTE_ASSET_AMOUNT,$QUOTE_POUND_VALUE,'$EXCHANGE_NAME',$EXCHANGE_TRANSACTION_ID);"
    echo "Done"
fi




