import asyncio
import base64
import requests

from pathlib import Path
from multiversx_sdk_core import Address, Token, TokenTransfer, TokenComputer, TransactionComputer, ContractQueryBuilder, AccountNonceHolder
from multiversx_sdk_wallet.user_signer import UserSigner
from multiversx_sdk_network_providers import ProxyNetworkProvider, ApiNetworkProvider
from multiversx_sdk_core.transaction_factories import SmartContractTransactionsFactory

provider = ProxyNetworkProvider("https://testnet-gateway.multiversx.com")
config = provider.get_network_config()

api_url = f'https://testnet-api.multiversx.com'
network_provider = ApiNetworkProvider(f"{api_url}")

contract_address = Address.from_bech32("erd1qqqqqqqqqqqqqpgqlffq8ed0clveymt9kzhwpxxahnpe6vher8qshlrtte")

sc_factory = SmartContractTransactionsFactory(config, TokenComputer())
transaction_computer = TransactionComputer()

signer = UserSigner.from_pem_file(Path("/home/elrond/SmartContracts/walletKey.pem"))
pemWallet = Address.from_bech32("erd18lsmq9rldm52syrgqzpwrjrvqlsxprgvp9v6ne5qtjymqgzgr8qs9ngtcl")
account_on_network = provider.get_account(pemWallet)
nonce_holder = AccountNonceHolder(account_on_network.nonce)

async def transaction():
    call_transaction = sc_factory.create_transaction_for_execute(
        sender=pemWallet,
        contract=contract_address,
        function="getRandomNumberSC",
        gas_limit=60000000,
        arguments=[ 1, 100 ], 
        token_transfers=[  ]
    )

    call_transaction.nonce = nonce_holder.get_nonce_then_increment()
    call_transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(call_transaction))

    #print("Transaction:", call_transaction.__dict__)
    #print("Transaction data:", call_transaction.data)

    hash = provider.send_transaction(call_transaction)
    #print("Transaction hash:", hash)
    return hash

async def query():
    global contract_address
    global network_provider

    builder = ContractQueryBuilder(
        contract=contract_address,
        function="getDynamicStatsSubtypeCount",
        call_arguments=[ "Necklace" ],
        caller=pemWallet
    )

    query = builder.build()
    response = network_provider.query_contract(query)
    if response.return_code == "ok":
        print(f"{response.return_data}")

async def api_call(data):
    global global_count

    url = f"{api_url}/{data}"

    response = requests.get(url, headers={"Content-Type":"application/json"})
    if response.status_code == 200:
        data = response.json()

        if data["status"] and data["status"] == "success" and "operations" in data and data["operations"] and data["operations"][0] and data["operations"][0]["data"]:
            message = data["operations"][0]["data"]

            values = message.split('@')[1:]

            output = ""
            for i in range(10):
                gotvalue = int(values[i + 1], 16)
                output = f"{output}{gotvalue} "
                if gotvalue > 70:
                    global_count += 1

            print(output)
            return True
        return False
    #else:
        #print("Eroare la interogarea API-ului", response.status_code)

global_count = 0
async def main():
    i = 0
    while (i < 10):
        hash = await transaction()
        while (not await api_call(f"transactions/{hash}")):
            await asyncio.sleep(1)
        i += 1
    print(f"Iteme reusite la +7: {global_count}")

    #await query()

if __name__ == '__main__':
    asyncio.run(main())
