#!/usr/bin/zsh

set -e

echo -n "DATE_TIME_ZONE: "
read DATE_TIME_ZONE
echo -n "ASSET_NAME: "
read ASSET_NAME
echo -n "ASSET_AMOUNT: "
read ASSET_AMOUNT
echo -n "POUND_TAX: "
read POUND_TAX
echo -n "POUND_VALUE: "
read POUND_VALUE
echo -n "DOLLAR_VALUE: "
read DOLLAR_VALUE
echo -n "COMPANY_NAME: "
read COMPANY_NAME
echo -n "TRANSACTION_ID: "
read TRANSACTION_ID

if [[ $TRANSACTION_ID != NULL ]]; then
    TRANSACTION_ID="'$TRANSACTION_ID'"
fi

echo "INSERT: $DATE_TIME_ZONE, $ASSET_NAME, $ASSET_AMOUNT, $POUND_TAX, $POUND_VALUE, $DOLLAR_VALUE, $COMPANY_NAME, $TRANSACTION_ID"
echo -n "[y/N]: "
read ANSWER
if [[ $ANSWER == 'y' ]]; then
    psql -h server.lan -p 5432 -U admin -d taxes --command="INSERT INTO salary(date_time_zone,asset_name,asset_amount,pound_tax,pound_value,dollar_value,company_name,transaction_id) VALUES ('$DATE_TIME_ZONE','$ASSET_NAME',$ASSET_AMOUNT,$POUND_TAX,$POUND_VALUE,$DOLLAR_VALUE,'$COMPANY_NAME',$TRANSACTION_ID);"
    echo "Done"
fi




