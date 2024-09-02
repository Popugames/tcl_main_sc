#USER_PEM="/home/elrond/SmartContracts/walletKey.pem"
USER_PEM="/home/elrond/SmartContracts/tcl-deployer.pem"

#PROXY="https://devnet-gateway.multiversx.com"
#PROXY="https://testnet-gateway.multiversx.com"
PROXY="https://gateway.multiversx.com"

#CHAIN_ID="D"
#CHAIN_ID="T"
CHAIN_ID="1"

#TOKEN_ID="XTCL-a8e1ec" #devnet
#TOKEN_ID="XTCL-629e44" #testnet
TOKEN_ID="TCL-fe459d" #mainnet


SC_ADDRESS=erd1qqqqqqqqqqqqqpgqm77vv5dcqs6kuzhj540vf67f90xemypd0ufsygvnvk
#SC_ADDRESS=erd1qqqqqqqqqqqqqpgq0let8nafpuzwstfltsq7sfq8xhgljk3wr8qsspcvz2 #testnet
#SC_ADDRESS=erd1qqqqqqqqqqqqqpgqm77vv5dcqs6kuzhj540vf67f90xemypd0ufsygvnvk #mainnet

USER_ADDRESS="erd1tpayjteeg67rq7me94k36705dh2c077xjsmhzdmkkwjeg0w00ufsmmltyc"
#USER_ADDRESS="erd1as8u30zlk7n6c3kvcwkegwq5ujmymaadlwyedcj0ywsp785wyt9q36h0n0"
#USER_ADDRESS="erd18lsmq9rldm52syrgqzpwrjrvqlsxprgvp9v6ne5qtjymqgzgr8qs9ngtcl"

TEAM_WALLET="erd16sm8ez0f8f8t4cuxzspsxmfrt0wkst6gc0zntazu0ef9nyjw9nmsrakr8k"
#NEW_USER_ADDRESS="erd1tpayjteeg67rq7me94k36705dh2c077xjsmhzdmkkwjeg0w00ufsmmltyc"

SERVER_WALLET="erd18lsmq9rldm52syrgqzpwrjrvqlsxprgvp9v6ne5qtjymqgzgr8qs9ngtcl"
#SERVER_WALLET="erd18lsmq9rldm52syrgqzpwrjrvqlsxprgvp9v6ne5qtjymqgzgr8qs9ngtcl" #testnet
#SERVER_WALLET="erd1tpayjteeg67rq7me94k36705dh2c077xjsmhzdmkkwjeg0w00ufsmmltyc" #mainnet


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

NFT_NONCE=6

deploy() {
    mxpy --verbose contract deploy \
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
    --bytecode="/home/elrond/SmartContracts/tcl_main_sc/output/tcl_main_sc.wasm" \
    --recall-nonce --pem=${USER_PEM} \
    --metadata-payable \
    --gas-limit=200000000 \
    --send --outfile="upgrade-devnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --arguments ${TOKEN_ID_HEX} || return
}

COLLECTION_ID="TCLBOOST-88ee35" #------------------------------------------------------------------------------------1
COLLECTION_ID_HEX="0x$(echo -n ${COLLECTION_ID} | xxd -p -u | tr -d '\n')"
NFT_NAME="Boost" #----000000000000000--------------------------------------------------------------------------------2
NFT_NAME_HEX="0x$(echo -n ${NFT_NAME} | xxd -p -u | tr -d '\n')"
NFT_PRICE=10000000000000000000000 #1000000000000000000000 = 1000
NFT_MAX=365 #---------------------------------------------------------------------------------------------------------3
ROYALTIES=1000 #1000=10%
MINT_TOKEN_ID=${TOKEN_ID_HEX}
NEED_SOCKET=false #---------------------------------------------------------------------------------------------------4

#---------------------------------------------------------------------------------------------------------------------5
IMAGE_CID=("QmWP7saJZf3P966QNq15844rn6FFyuGuKAyMAVUciWhBCP" "QmTyPE3WZ7EXdQgXa2k9HokVK9TbigsNvUZNbgxUr9SDfx" "QmXtBcLAE3w799SEkAmrc17UQNfU93eEiAooevVpMgLjPt")

#---------------------------------------------------------------------------------------------------------------------6
METADATA_CID=("QmaDdkyorz7V16htXuQskMGn1jEpfamNNNQZ3UuAFTLfV7" "QmaDdkyorz7V16htXuQskMGn1jEpfamNNNQZ3UuAFTLfV7" "QmdBtMrAd8rJVw1cgGG6KHoegcYV2aeMKzKrNxPDUP6ZWB")

NFT_COUNT=(0 0 0) #limited by NFT_MAX
MAX_SOCKET=(0 0 0) #----------------------------------------------------------------------------------------------------7
MAX_CRYSTAL_VARIANTS=(0 0 0) # index 0 based (last folder index+1) #----------------------------------------------------8
MAX_BONUS_VARIANTS=(0 0 0)  # index 0 based (last json index+1)#--------------------------------------------------9
EQUIP_SLOT=8 # Armor=0, Weapon=1, Shield=2, Helmet=3, Necklace=4, Bracelet=5, Shoes=6, Earrings=7, Boost=8 --------------------10

COLLECTION_DATA=""
for ((i = 0; i < ${#METADATA_CID[@]}; i++))
do
    METADATA_CID_HEX=$(echo -n ${METADATA_CID[$i]} | xxd -p -u | tr -d '\n')
    IMAGE_CID_HEX=$(echo -n ${IMAGE_CID[$i]} | xxd -p -u | tr -d '\n')
    COLLECTION_DATA="${COLLECTION_DATA}0x${IMAGE_CID_HEX} 0x${METADATA_CID_HEX} ${NFT_COUNT[$i]} ${MAX_SOCKET[$i]} ${MAX_CRYSTAL_VARIANTS[$i]} ${MAX_BONUS_VARIANTS[$i]} "
done


COIN_COUNT=(200 440 1200 2700 5900 13000)
TCL_PRICE=(320 640 1600 3200 6400 12800)

COIN_PACKS=""
for ((i = 0; i < ${#COIN_COUNT[@]}; i++))
do
    COIN_PACKS="${COIN_PACKS} ${COIN_COUNT[i]} ${TCL_PRICE[i]}000000000000000000 "
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
    #sleep 12
    #startMinting
    #echo "Running startMinting"
}



distributeTokenPurchases() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="distributeTokenPurchases" \
    --arguments "ACESTA FUNCTIE E TOMPORARA SI TREBUIE SCOASA"
}




setCoinPacks() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="setCoinPacks" \
    --arguments ${COIN_PACKS}
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
    collection_id=str:TCLBOOST-88ee35
    sft_nonce=3
    sft_price=5000000000000000000000
    sft_sold=58 #24
    sft_max=250 #78
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

CLAIM_BORROWING_REWARDS() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="claimBorrowingRewards" \
    --arguments ${USER_ADDRESS}
}

BORROW_NFT() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="borrowNft" \
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
TCL_PRICE=5000000000000000
APR_MAX=400
MIN_AMOUNT_TO_BORROW=2000000000000000000000

SET_global_props() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="setGlobalProps" \
    --arguments ${TOKEN_ID_HEX} ${NFT_UPGRADE_PRICE} ${ADD_BONUS_PRICE} ${CHANGE_BONUS_PRICE} ${ADD_SOCKET_PRICE} ${ADD_CRYSTAL_PRICE} $CHANGE_CRYSTAL_PRICE ${NFT_UPGRADE_CHANCE} ${ADD_REFINEMENT_PRICE} $TCL_PRICE $TEAM_WALLET $SERVER_WALLET $APR_MAX $MIN_AMOUNT_TO_BORROW
}

getCoinPacks() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCoinPacks"
}

getReferralData() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReferralData" \
     --arguments $USER_ADDRESS
}

getRewardsData() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRewardsData" \
     --arguments $USER_ADDRESS
}

getUserBorrowedAmount() {
    epoch=671
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserBorrowedAmount" \
     --arguments $USER_ADDRESS $epoch
}

getUserLoanedAmount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserLoanedAmount" \
     --arguments $USER_ADDRESS
}

getLendingData() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getLendingData" \
     --arguments $USER_ADDRESS
}

getHasCrystal() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getHasCrystal" \
     --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getCrystalCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCrystalCount" \
     --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getCrystalCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCrystalCount" \
     --arguments $COLLECTION_ID_HEX $NFT_NONCE
}

getNftCrystalVariant() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNftCrystalVariant" \
     --arguments $COLLECTION_ID_HEX $NFT_NONCE
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
    nft_token_id=str:$COLLECTION_ID
    nft_nonce=$NFT_NONCE
    nft_amount=1
    method_name=str:equipNft

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

LOAN_NFT() {
    receiver=$SC_ADDRESS
    tokens_send_count=2
    method_name=str:loanNft
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
    nft_slot=6
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

BUY_COINS() {
    method_name=str:buyCoins
    payment_token_id=str:$TOKEN_ID
    payment_amount=320000000000000000000 #1000000000000000000 = 1TCL
    coins_amount=200

    mxpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${USER_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="ESDTTransfer" \
        --arguments \
            $payment_token_id \
            $payment_amount \
            $method_name \
            $coins_amount \
        --send || return
}



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

ADD_BOOST_STAKING() {
    receiver=$SC_ADDRESS
    tokens_send_count=1
    method_name=str:addBoostStaking
    payment_token_id=str:$TOKEN_ID
    payment_token_nonce=0
    payment_amount=150000000000000000000000 #1000000000000000000 = 1TCL
  
    mxpy --verbose contract call ${SERVER_WALLET} \
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
            $USER_ADDRESS\
        --send || return
}

REMOVE_BOOST_STAKING() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --function="removeBoostStaking" \
    --arguments ${USER_ADDRESS}
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
    payment_amount=500000000000000000000000000 #1000000000000000000 = 1TCL
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
    nonce=41
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