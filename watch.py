import json
import subprocess
import time

import Crypto
from Crypto.PublicKey import RSA
from Crypto import Random
import ast

c_url = 'http://127.0.0.1:8114'
c_ckb_cli = '/mnt/sata/ckb_testnet/ckb-cli'
c_code_hash = '0x3418472a01e7b2970bce2123ca9beb5db7c357d8b7d9fbb0f491c875552fa913'
c_args = '0x32e555f3ff8e135cece1351a6a2971518392c1e30375c1e006ad0ce8eac07947'
c_hash_type = 'data'
c_tx_list = './tx.list'
c_private_key = ''


def get_tip_number():
    r = subprocess.getoutput(f'{c_ckb_cli} --url {c_url} rpc get_tip_block_number')
    return int(r)


def get_block_by_number(number: int):
    r = subprocess.getoutput(
        f'{c_ckb_cli} --url {c_url} --output-format json rpc get_block_by_number --number {number}')
    return json.loads(r)


def get_transaction(hash: str):
    r = subprocess.getoutput(f'{c_ckb_cli} --url {c_url} --output-format json rpc get_transaction --hash {hash}')
    return json.loads(r)


def handle_block(block):
    for tx in block['transactions']:
        handle_transaction(tx)


def handle_transaction(tx):
    # {
    #     "cell_deps": [],
    #     "hash": "0xe9780ec915dbbcf8f285c54ae128e1f7623808590f3334949f34937473877018",
    #     "header_deps": [],
    #     "inputs": [
    #         {
    #             "previous_output": {
    #                 "index": 4294967295,
    #                 "tx_hash": "0x0000000000000000000000000000000000000000000000000000000000000000"
    #             },
    #             "since": "0x4e20 (absolute block(20000))"
    #         }
    #     ],
    #     "outputs": [
    #         {
    #             "capacity": "2009.76800007",
    #             "lock": {
    #                 "args": "0xc8328aabcd9b9e8e64fbc566c4385c3bdeb219d7",
    #                 "code_hash": "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8 (sighash)",
    #                 "hash_type": "type"
    #             },
    #             "type": {
    #
    #             }
    #         }
    #     ],
    #     "outputs_data": [
    #         "0x"
    #     ],
    #     "version": 0,
    #     "witnesses": [
    #         "0x590000000c00000055000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce80114000000c8328aabcd9b9e8e64fbc566c4385c3bdeb219d700000000"
    #     ]
    # }

    input_tx_hash = tx['inputs'][0]['previous_output']['tx_hash']
    input_index = tx['inputs'][0]['previous_output']['index']
    if input_tx_hash == '0x0000000000000000000000000000000000000000000000000000000000000000':
        return
    from_addr = get_transaction(input_tx_hash)['outputs'][input_index]['type']['args']

    for output in tx['outputs']:
        if output['type']['code_hash'] == c_code_hash \
                and output['type']['args'] == c_args \
                and output['type']['hash_type'] == c_hash_type:

            data = bytes.fromhex(tx['outputs_data'])
            encrypted_amount_nonce = data[32:]
            key = RSA.import_key(c_private_key)
            decrypted = key.decrypt(str(encrypted_amount_nonce))
            amount_data = decrypted[:16]
            amount = int.from_bytes(amount_data, byteorder='little')

            l = {
                'from': from_addr,
                'to': output['lock']['args'],
                'amount': amount,
            }

            print(json.dumps(l))
            with open(c_tx_list, 'w') as f:
                f.write(json.dumps(l))


def main():
    number = 20000
    for _ in range(1 << 32):
        tip_number = get_tip_number()
        while number < tip_number:
            block = get_block_by_number(number)
            print(block['header']['number'])
            handle_block(block)
            number += 1
        time.sleep(1)


if __name__ == "__main__":
    main()
