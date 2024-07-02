USER_PEM="/home/elrond/SmartContracts/walletKey.pem"
#USER_PEM="/home/elrond/SmartContracts/walletShard2.pem"

#PROXY="https://devnet-gateway.multiversx.com"
PROXY="https://testnet-gateway.multiversx.com"
#PROXY="https://gateway.multiversx.com"

#CHAIN_ID="D"
CHAIN_ID="T"
#CHAIN_ID="1"

#TOKEN_ID="XTCL-a8e1ec" #devnet
TOKEN_ID="XTCL-467992" #testnet
#TOKEN_ID="TCL-fe459d" #mainnet


SC_ADDRESS=erd1qqqqqqqqqqqqqpgqsk0tqdpd4mx728k0uece45pahh4u0p03r8qszmfadm
#SC_ADDRESS=erd1qqqqqqqqqqqqqpgq4cr05ac4zzrxu7f5vx83ktg5czyc9wp2ln5sl2w0r0 #devnet

USER_ADDRESS="erd18lsmq9rldm52syrgqzpwrjrvqlsxprgvp9v6ne5qtjymqgzgr8qs9ngtcl"
#USER_ADDRESS="erd1ajdkdj0lj4t29747uhv6mzfr7y3wdf7sk8hxea6g257cvs7vcj9qhpnefd"

TEAM_WALLET="erd18lsmq9rldm52syrgqzpwrjrvqlsxprgvp9v6ne5qtjymqgzgr8qs9ngtcl"
#NEW_USER_ADDRESS="erd1tpayjteeg67rq7me94k36705dh2c077xjsmhzdmkkwjeg0w00ufsmmltyc"

SERVER_WALLET="erd18lsmq9rldm52syrgqzpwrjrvqlsxprgvp9v6ne5qtjymqgzgr8qs9ngtcl"
#NEW_USER_ADDRESS="erd1tpayjteeg67rq7me94k36705dh2c077xjsmhzdmkkwjeg0w00ufsmmltyc"


TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
TOKEN_ID_ONLY_HEX="$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

REFERRAL_CODE="LANDER23"
REFERRAL_CODE_HEX="0x$(echo -n ${REFERRAL_CODE} | xxd -p -u | tr -d '\n')"



ADD_TOKEN_FUNCTION="addTokensBalance"
ADD_TOKEN_FUNCTION_ONLY_HEX="$(echo -n ${ADD_TOKEN_FUNCTION} | xxd -p -u | tr -d '\n')"

EGLD_PRICE=38700000000000000000 #40$ = 40000000000000000000
USD_VALUE_TO_REFUND=3300000000000000000
            
TOKEN_PURCHASED=200000000000000000000000 #1000000000000000000000000 = 1M
VESTED_PERCENT=5 #5% VESTED TOKENS ON BUY

ITEM_TYPE="str:Weapon 2 str:Armor 2"
ADD_BONUS_CHANCE="100 80 60 50 30 0 0"


DYNAMIC_BONUSES=("Max HP" "Max SP" "Max SP" "Vitality" "Intelligence" "Strength" "Dexterity" "Attack Speed" "Movement Speed" "Spell Speed" "Damage will be absorbed by HP" "Damage will be absorbed by SP" "Chance to take SP from the enemy" "HP Regeneration" "SP Regeneration" "Poisoning Chance" "Chance for Blackout" "Slowing Chance" "Chance of Critical Hit" "Chance of Piercing Hit" "Strong Against Half Humans" "Strong Against Monsters" "Chance to block physical attacks" "Chance to reflect physical attacks" "Chance of Evading Arrow" "Sword Defence" "Dagger Defence" "Fan Defence" "Staff Defence" "Arrow Resistance" "Magic Resistance"  "Poison Resistance" "Chance of double EXP" "Chance of double Curse drop" "Chance of double Item drop" "Defence against Blackouts" "Defence against Slowing" "Attack Value" "Average Damage" "Spell Damage")
DYNAMIC_STATS_CHANCES=""
counter=1
for DYNAMIC_BONUS in "${DYNAMIC_BONUSES[@]}"; do
    hex_value=$(echo -n "$DYNAMIC_BONUS" | xxd -p -u | tr -d '\n')
    DYNAMIC_STATS_CHANCES="${DYNAMIC_STATS_CHANCES}0x${hex_value} ${counter} "
    counter=$((counter + 1))
done


DYNAMIC_VALUES=("Max HP" 1 2 3 "Max SP" 1)
DYNAMIC_STATS_VALUES=""
for DYNAMIC_VALUE in "${DYNAMIC_VALUES[@]}"; do
    if [[ "$DYNAMIC_VALUE" =~ ^[0-9]+$ ]]; then
        DYNAMIC_STATS_VALUES="${DYNAMIC_STATS_VALUES}${DYNAMIC_VALUE} "
    else
        hex_value=$(echo -n "$DYNAMIC_VALUE" | xxd -p -u | tr -d '\n')
        DYNAMIC_STATS_VALUES="${DYNAMIC_STATS_VALUES}0x${hex_value} "
    fi
done

NFT_NONCE=1

deploy() {
    mxpy --verbose contract deploy\
    --bytecode="/home/elrond/SmartContracts/tcl_main_sc/output/tcl_main_sc.wasm" \
    --recall-nonce --pem=${USER_PEM} \
    --metadata-payable \
    --gas-limit=200000000 \
    --send --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --arguments ${TOKEN_ID_HEX} || return
}
upgrade() {
    mxpy --verbose contract upgrade ${SC_ADDRESS} \
    --bytecode="/home/elrond/SmartContracts/tcl_main_sc/output/tcl_main_sc.wasm"\
    --recall-nonce --pem=${USER_PEM} \
    --metadata-payable \
    --gas-limit=200000000 \
    --send --outfile="upgrade-devnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --arguments ${TOKEN_ID_HEX} || return
}

COLLECTION_ID="TCLMOUNT-3e5536" #------------------------------------------------------------------------------------1
COLLECTION_ID_HEX="0x$(echo -n ${COLLECTION_ID} | xxd -p -u | tr -d '\n')"
NFT_NAME="Necklace" #----000000000000000--------------------------------------------------------------------------------2
NFT_NAME_HEX="0x$(echo -n ${NFT_NAME} | xxd -p -u | tr -d '\n')"
NFT_PRICE=1000000000000000000000 #1000000000000000000000 = 1000
NFT_MAX=300 #---------------------------------------------------------------------------------------------------------3
ROYALTIES=500 #1000=10%
MINT_TOKEN_ID=${TOKEN_ID_HEX}
NEED_SOCKET=true #---------------------------------------------------------------------------------------------------4

#---------------------------------------------------------------------------------------------------------------------5
IMAGE_CID=("QmQ1TLhzL9iPPGuhcHwhJBfJ2wpv1uiA8SYoMLVa5Rc1YA" "QmQ1TLhzL9iPPGuhcHwhJBfJ2wpv1uiA8SYoMLVa5Rc1YA")

#---------------------------------------------------------------------------------------------------------------------6
METADATA_CID=("QmQ2YqsNeURj4MQAw41nhCN4J3xm3eRrMRvvEL8i9SwN7a" "QmQ2YqsNeURj4MQAw41nhCN4J3xm3eRrMRvvEL8i9SwN7a")

NFT_COUNT=(300 200) #limited by NFT_MAX
MAX_SOCKET=(3 3) #----------------------------------------------------------------------------------------------------7
MAX_CRYSTAL_VARIANTS=(1 1) # index 0 based (last folder index+1) #----------------------------------------------------8
MAX_BONUS_VARIANTS=(1000 1000)  # index 0 based (last json index+1)#--------------------------------------------------9
EQUIP_SLOT=4 # Armor=0, Weapon=1, Shield=2, Helmet=3, Necklace=4, Bracelet=5, Shoes=6, Earrings=7 --------------------10

COLLECTION_DATA=""
for ((i = 0; i < ${#METADATA_CID[@]}; i++))
do
    METADATA_CID_HEX=$(echo -n ${METADATA_CID[$i]} | xxd -p -u | tr -d '\n')
    IMAGE_CID_HEX=$(echo -n ${IMAGE_CID[$i]} | xxd -p -u | tr -d '\n')
    COLLECTION_DATA="${COLLECTION_DATA}0x${IMAGE_CID_HEX} 0x${METADATA_CID_HEX} ${NFT_COUNT[$i]} ${MAX_SOCKET[$i]} ${MAX_CRYSTAL_VARIANTS[$i]} ${MAX_BONUS_VARIANTS[$i]} "
done


SET_Collection_INIT() {

    SET_Collection
    echo "Running SET_Collection"
    sleep 12
    TRANSFER_CREATE_ROLE
    echo "Running TRANSFER_CREATE_ROLE"
    sleep 12
    SET_SPECIAL_ROLE
    echo "Running SET_SPECIAL_ROLE"
    sleep 12
    startMinting
    echo "Running startMinting"
}




SET_Collection() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="setCollection" \
    --arguments ${COLLECTION_ID_HEX} $NFT_NAME_HEX ${NFT_PRICE} ${NFT_MAX} ${ROYALTIES} $NEED_SOCKET ${EQUIP_SLOT} ${COLLECTION_DATA}
}

SET_SFT() {
    collection_id=str:TCLMOUNT-3e5536
    sft_nonce=4
    sft_price=2500000000000000000000
    sft_sold=400
    sft_max=400
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="setSft" \
    --arguments $collection_id $sft_nonce $sft_price $sft_sold $sft_max
}

CLAIM_REWARDS() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="claimRewards" \
    --arguments ${USER_ADDRESS}
}

NFT_UPGRADE_PRICE=150000000000000000000 #1000000000000000000 = 1TCL
ADD_BONUS_PRICE=300000000000000000000
CHANGE_BONUS_PRICE=150000000000000000000
ADD_SOCKET_PRICE=150000000000000000000
ADD_CRYSTAL_PRICE=300000000000000000000
CHANGE_CRYSTAL_PRICE=300000000000000000000
NFT_UPGRADE_CHANCE=0
ADD_REFINEMENT_PRICE=150000000000000000000
TCL_PRICE=6000000000000000
APR_MAX=500

SET_global_props() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="setGlobalProps" \
    --arguments ${TOKEN_ID_HEX} ${NFT_UPGRADE_PRICE} ${ADD_BONUS_PRICE} ${CHANGE_BONUS_PRICE} ${ADD_SOCKET_PRICE} ${ADD_CRYSTAL_PRICE} $CHANGE_CRYSTAL_PRICE ${NFT_UPGRADE_CHANCE} ${ADD_REFINEMENT_PRICE} $TCL_PRICE $TEAM_WALLET $SERVER_WALLET $APR_MAX
}

getSftSold() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSftSold" \
     --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getSftMax() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSftMax" \
     --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getNftQuality() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftQuality" \
     --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getNftsMinted() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftsMinted" \
     --arguments $COLLECTION_ID_HEX
}

getTotalStakedAmount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTotalStakedAmount"
}

buildUrisVec() {
    current_wave=1
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="buildUrisVec" \
    --arguments $COLLECTION_ID_HEX $current_wave
}

getUserStakedAmount() {
   
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserStakedAmount" \
    --arguments $USER_ADDRESS 
}

getAprMax() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getAprMax"
}

calculateReward() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="calculateReward" \
    --arguments $USER_ADDRESS
}

getTclCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTclCount" \
    --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getEquippedNfts() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getEquippedNfts" \
    --arguments $USER_ADDRESS
}

getNftsData() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftsData" \
    --arguments $USER_ADDRESS $COLLECTION_ID_HEX $NFT_NONCE
}

getTclMax() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTclMax" \
    --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getRoles() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRoles" \
    --arguments $COLLECTION_ID_HEX
}

getTotalReserveAmount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTotalReserveAmount"
}

isPaused() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="isPaused" \
    --arguments $COLLECTION_ID_HEX
}

getReferralOwner() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReferralOwner" \
    --arguments $REFERRAL_CODE_HEX
}

PAYD_X2=3000000000000000000000
calculateStorage() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="calculateStorage" \
    --arguments $PAYD_X2
}

getBonusVariant() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getBonusVariant" \
    --arguments ${COLLECTION_ID_HEX} ${NFT_NONCE}
}

getNftBonusVariant() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftBonusVariant" \
    --arguments ${COLLECTION_ID_HEX} ${NFT_NONCE}
}


getNftUpgradePrice() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftUpgradePrice"
}

getPaymentTokenId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftUpgradePrice"
}


getAttributesBuffer() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getAttributesBuffer" \
    --arguments ${IMAGE_CID_HEX} true true 1 1 1
}

getNewNftQuality() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNewNftQuality" \
    --arguments 0
}

GET_RandomNumber() {
    nonce=28
    quantity=1
    mxpy --verbose contract call $SC_ADDRESS --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="getRandomNumberSC" \
        --arguments 1 100 \
        --send || return
}

getRandomNumber() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRandomNumber" \
    --arguments 0 4
}

getPrivateSeed() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getPrivateSeed"
}

getCurrentWave() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCurrentWave" \
    --arguments ${COLLECTION_ID_HEX}
}
getCollectionNonce() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCollectionNonce" \
    --arguments ${COLLECTION_ID_HEX}
}

getImageCid() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getImageCid" \
    --arguments ${COLLECTION_ID_HEX}
}

getMetadataCid() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getMetadataCid" \
    --arguments ${COLLECTION_ID_HEX}
}

startMinting() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="startMinting" \
    --arguments ${COLLECTION_ID_HEX}
}

pauseMinting() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="pauseMinting" \
    --arguments ${COLLECTION_ID_HEX}
}

getCid() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCid" \
    --arguments ${COLLECTION_ID_HEX} 1
}

MINT_NFT_FUNTION="0x$(echo -n "mintNft" | xxd -p -u | tr -d '\n')"

MINT_NFT() {
    method_name=str:mintNft
    my_token=str:$TOKEN_ID
    token_amount=$NFT_PRICE
    mxpy --verbose contract call ${SC_ADDRESS} --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="ESDTTransfer" \
        --arguments  $my_token $token_amount $method_name $COLLECTION_ID_HEX \
        --send || return
}

# Initialize transaction counter
transaction_count=0
max_transactions=1000
UPGRADE_NFT() {
    receiver=$SC_ADDRESS
    tokens_send_count=2
    method_name=str:upgradeNft
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=150000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $nft_token_id \
            $nft_nonce \
            $nft_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

UPGRADE_EQUIPPED_NFT() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:upgradeNft
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=150000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

EQUIP_NFT() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:equipNft
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $nft_token_id \
            $nft_nonce \
            $nft_amount \
            $method_name \
        --send || return
}

EQUIP_NFT_TCL() {
    receiver=$SC_ADDRESS
    tokens_send_count=2
    method_name=str:equipNft
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=112233445566778899000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $nft_token_id \
            $nft_nonce \
            $nft_amount \
            $method_name \
        --send || return
}

UNEQUIP_NFT() {
    nft_slot=5
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="unequipNft" \
    --arguments ${nft_slot}
}

SET_REFERRAL_OWNER() {
    mxpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="setReferralCodeOwner" \
        --arguments $REFERRAL_CODE_HEX \
        --send || return
}
# Loop to send transactions every second

# Loop to send transactions
# while [ $transaction_count -lt $max_transactions ]; do
#     UPGRADE_NFT
#     if [ $? -eq 0 ]; then
#         ((transaction_count++))
#         echo "Transaction count: $transaction_count"
#     else
#         echo "Failed to send transaction"
#     fi
#     sleep 3
# done

ADD_TCL() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:addTcl
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=112233445566778899000 #1000000000000000000 = 1TCL
    collection_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $method_name \
            $USER_ADDRESS \
            $collection_id \
            $nft_nonce \
        --send || return
}


ADD_BONUS() {
    receiver=$SC_ADDRESS
    tokens_send_count=2
    method_name=str:addBonus
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=300000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $nft_token_id \
            $nft_nonce \
            $nft_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

ADD_BONUS_EQUIPPED() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:addBonus
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=300000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

CHANGE_BONUS() {
    receiver=$SC_ADDRESS
    tokens_send_count=2
    method_name=str:changeBonus
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=150000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $nft_token_id \
            $nft_nonce \
            $nft_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

CHANGE_BONUS_EQUIPPED() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:changeBonus
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=150000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

ADD_CRYSTAL() {
    receiver=$SC_ADDRESS
    tokens_send_count=2
    method_name=str:addCrystal
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=300000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $nft_token_id \
            $nft_nonce \
            $nft_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

ADD_CRYSTAL_EQUIPPED() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:addCrystal
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=300000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}


CHANGE_CRYSTAL() {
    receiver=$SC_ADDRESS
    tokens_send_count=2
    method_name=str:changeCrystal
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=300000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $nft_token_id \
            $nft_nonce \
            $nft_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

CHANGE_CRYSTAL_EQUIPPED() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:changeCrystal
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=300000000000000000000 #1000000000000000000 = 1TCL
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1

    mxpy --verbose contract call ${USER_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments \
            $receiver \
            $tokens_send_count \
            $payment_token_id \
            $payment_token_nonce \
            $payment_amount \
            $method_name \
            $nft_token_id \
            $nft_nonce \
        --send || return
}

ADD_RESERVE() {
    payment_amount=1000000000000000000000000 #1000000000000000000 = 1TCL
    method_name=addReserve
    method_name_HEX="$(echo -n ${method_name} | xxd -p -u | tr -d '\n')"
    mxpy --verbose tx new \
    --receiver ${SC_ADDRESS} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=10000000 \
    --data="ESDTTransfer@${TOKEN_ID_ONLY_HEX}@d3c21bcecceda1000000@${method_name_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

getNftBonusVariant() {
    nft_nonce=$NFT_NONCE
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftBonusVariant" \
    --arguments ${COLLECTION_ID_HEX} $nft_nonce
}
getBonusCount() {
    nft_nonce=$NFT_NONCE
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getBonusCount" \
    --arguments ${COLLECTION_ID_HEX} $nft_nonce
}
getHasBonus() {
    nft_nonce=$NFT_NONCE
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getHasBonus" \
    --arguments ${COLLECTION_ID_HEX} $nft_nonce
}

TRANSFER_CREATE_ROLE() {
    my_token=str:$COLLECTION_ID
    main_contract="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"
    mxpy --verbose contract call $main_contract --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="transferNFTCreateRole" \
        --arguments $COLLECTION_ID_HEX $USER_ADDRESS $SC_ADDRESS \
        --send || return
}

SET_SPECIAL_ROLE() {
    my_token=str:$COLLECTION_ID
    role=str:ESDTRoleNFTUpdateAttributes      #ESDTRoleNFTBurn
    main_contract="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"
    mxpy --verbose contract call $main_contract --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="setSpecialRole" \
        --arguments $COLLECTION_ID_HEX $SC_ADDRESS $role \
        --send || return
}

BURN_NFT() {
    nonce=17
    quantity=1
    mxpy --verbose contract call $USER_ADDRESS --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="ESDTNFTBurn" \
        --arguments $COLLECTION_ID_HEX $nonce $quantity \
        --send || return
}

BURN_NFT_BULK() {
    nonce=5

    for ((i = 1; i < 34; i++))
    do
     quantity=1
    mxpy --verbose contract call $USER_ADDRESS --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="ESDTNFTBurn" \
        --arguments $COLLECTION_ID_HEX $i $quantity \
        --send || return
    sleep 6
    done
}

getReferralOwner() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReferralOwner" \
    --arguments ${REFERRAL_CODE_HEX}
}
getReferralEarned() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReferralEarned" \
    --arguments ${REFERRAL_CODE_HEX}
}

getNftPrice() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftPrice" \
    --arguments ${COLLECTION_ID_HEX}
}

getNftMax() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftMax" \
    --arguments ${COLLECTION_ID_HEX}
}

getReferralCode() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReferralCode" \
    --arguments ${USER_ADDRESS}
}
getReferralCodeInvitee() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReferralCodeInvitee" \
    --arguments ${USER_ADDRESS}
}
getReferralInvitees() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReferralInvitees" \
    --arguments ${REFERRAL_CODE_HEX}
}
getTransactionsInvitees() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTransactionsInvitees" \
    --arguments ${REFERRAL_CODE_HEX}
}

##################################################################################

OWNER_changeReferralCode() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="changeReferralCode" \
    --arguments ${REFERRAL_CODE_HEX} ${USER_ADDRESS}
}

SET_ReferralCodeOwner() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="setReferralCodeOwner" \
    --arguments ${REFERRAL_CODE_HEX}
}
SET_ReferralCodeInvitee() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="setReferralCodeInvitee" \
    --arguments ${REFERRAL_CODE_HEX}
}

setDynamicStatsCount() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="setDynamicStatsCount" \
    --arguments 5
}
getDynamicStatsCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getDynamicStatsCount"
}
setDynamicStatsTypes() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="setDynamicStatsTypes" \
    --arguments ${ITEM_TYPE}
}
getDynamicStatsTypes() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getDynamicStatsTypes"
}
setAddBonusChance() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="setAddBonusChance" \
    --arguments ${ADD_BONUS_CHANCE}
}
getAddBonusChance() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getAddBonusChance"
}
setDynamicStatsChances() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="setDynamicStatsChances" \
    --arguments $(echo $DYNAMIC_STATS_CHANCES)
}
getDynamicStatsChances() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getDynamicStatsChances"
}


create_json() {
  jq -n \
  '[
    ["nested:str:Max HP", "u32:3", ["u16:1", "u16:2", "u16:3"]],
    ["nested:str:Max SP", "u32:1", ["u16:1"]]
  ]'
}

JSON_HEX="0x$(echo -n ${create_json} | xxd -p -u | tr -d '\n')"

setDynamicStatsValues() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="setDynamicStatsValues" \
    --arguments "str:buffer1|u16:1|u16:2|u16:3|str:buffer2|u16:4|u16:5|u16:6"
    #--arguments "${DYNAMIC_STATS_VALUES[@]}"
}
getDynamicStatsValues() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getDynamicStatsValues"
}


##################################################################################
getTotalEarnedAllReferrals() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTotalEarnedAllReferrals"
}
getTotalInviteesAllReferrals() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTotalInviteesAllReferrals"
}
getActiveReferralCodes() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getActiveReferralCodes"
}
getTotalReferralCodes() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTotalReferralCodes"
}
getTransactionsAllInvitees() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTransactionsAllInvitees"
}