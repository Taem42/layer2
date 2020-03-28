use bit_vec::BitVec;
use sha2::{Digest, Sha256};

use crate::utils::{convert_u128_to_bits, convert_u16_to_bits};
use crate::MerkleRoot;

struct Leaf {
    num:         u16,
    pub balance: u128,
}

impl Leaf {
    fn new(num: u16, balance: u128) -> Self {
        Leaf { num, balance }
    }

    fn set_balance(&mut self, balance: u128) {
        self.balance = balance;
    }

    fn transfer(&mut self, balance: u128) {
        assert!(self.balance >= balance);
        self.balance -= balance;
    }

    fn add(&mut self, balance: u128) {
        self.balance += balance;
    }

    fn hash(&self) -> MerkleRoot {
        let bits = convert_u16_to_bits(self.num);
        let mut one = bits_extend_to_256(bits);

        let bits = convert_u128_to_bits(self.balance);
        let mut two = bits_extend_to_256(bits);

        one.append(&mut two);
        assert!(one.len() == 512);

        let mut hasher = Sha256::new();
        hasher.input(bits_to_bytes(one));
        let ret = hasher.result();
        ret[..].to_vec()
    }
}

pub struct MerkleTree {
    leaves: Vec<Leaf>,
    root:   MerkleRoot,
}

impl MerkleTree {
    pub fn new() -> Self {
        let mut leaves = Vec::with_capacity(8);
        for i in 0 as u16..8 {
            leaves.push(Leaf::new(i, 0));
        }

        let root = recursion_merkle_hash(leaves.iter().map(|f| f.hash()).collect::<Vec<_>>());

        MerkleTree { leaves, root }
    }

    pub fn get_root(&mut self) -> MerkleRoot {
        self.root.clone()
    }

    pub fn set_balance(&mut self, index: usize, balance: u128) {
        assert!(index < 8);
        self.leaves[index].set_balance(balance);
        self.restore_root();
    }

    pub fn transfer(&mut self, from: usize, to: usize, balance: u128) {
        self.leaves[from].transfer(balance);
        self.leaves[to].add(balance);
        self.restore_root();
    }

    fn restore_root(&mut self) {
        self.root = recursion_merkle_hash(self.leaves.iter().map(|f| f.hash()).collect::<Vec<_>>());
    }
}

fn bits_extend_to_256(mut bits: Vec<bool>) -> Vec<bool> {
    assert!(bits.len() < 256);

    let len = 256 - bits.len();
    let mut temp = Vec::new();

    for _i in 0..len {
        temp.push(false);
    }

    bits.extend(temp.into_iter());
    bits
}

fn bits_to_bytes(bits: Vec<bool>) -> MerkleRoot {
    assert!(bits.len() % 8 == 0);
    let mut temp = BitVec::from_elem(bits.len(), false);

    for i in 0..bits.len() {
        temp.set(i, bits[i]);
    }

    temp.to_bytes()
}

fn merkle_two_hash(one: MerkleRoot, two: MerkleRoot) -> MerkleRoot {
    let mut temp = one;
    temp.extend(two.into_iter());

    let mut hasher = Sha256::new();
    hasher.input(temp);
    let ret = hasher.result();
    ret[..].to_vec()
}

fn recursion_merkle_hash(list: Vec<MerkleRoot>) -> MerkleRoot {
    let len = list.len();
    if len == 1 {
        list[0].clone()
    } else if len == 2 {
        merkle_two_hash(list[0].clone(), list[1].clone())
    } else {
        assert!(len == 4 || len == 8);
        let div = len / 2;
        let one = recursion_merkle_hash(list[0..div].to_vec());
        let two = recursion_merkle_hash(list[div..len].to_vec());
        recursion_merkle_hash(vec![one, two])
    }
}
