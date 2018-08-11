use std::vec::Vec;

mod native {
    extern "C" {
        pub fn ethereum_useGas(amount: u64);
        pub fn ethereum_getGasLeft() -> u64;
        pub fn ethereum_getAddress(resultOffset: *const u32);
        pub fn ethereum_getBalance(addressOffset: *const u32, resultOffset: *const u32);
        pub fn ethereum_getBlockCoinbase(resultOffset: *const u32);
        pub fn ethereum_getBlockDifficulty(resultOffset: *const u32);
        pub fn ethereum_getBlockGasLimit() -> u64;
        pub fn ethereum_getBlockHash(number: u64, resultOffset: *const u32) -> u32;
        pub fn ethereum_getBlockNumber() -> u64;
        pub fn ethereum_getBlockTimestamp() -> u64;
        pub fn ethereum_getTxGasPrice(valueOffset: *const u32);
        pub fn ethereum_getTxOrigin(resultOffset: *const u32);
        pub fn ethereum_log(
            dataOffset: *const u32,
            length: u32,
            numberOfTopics: u32,
            topic1: *const u32,
            topic2: *const u32,
            topic3: *const u32,
            topic4: *const u32,
        );
        pub fn ethereum_call(
            gas: u64,
            addressOffset: *const u32,
            valueOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_callCode(
            gas: u64,
            addressOffset: *const u32,
            valueOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_callDelegate(
            gas: u64,
            addressOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_callStatic(
            gas: u64,
            addressOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_create(
            valueOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
            resultOffset: *const u32,
        ) -> u32;
        pub fn ethereum_returnDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
        pub fn ethereum_getReturnDataSize() -> u32;
        pub fn ethereum_finish(dataOffset: *const u32, length: u32) -> !;
        pub fn ethereum_revert(dataOffset: *const u32, length: u32) -> !;
        pub fn ethereum_callDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
        pub fn ethereum_getCallDataSize() -> u32;
        pub fn ethereum_getCaller(resultOffset: *const u32);
        pub fn ethereum_getCallValue(resultOffset: *const u32);
        pub fn ethereum_codeCopy(resultOffset: *const u32, codeOffset: u32, length: u32);
        pub fn ethereum_getCodeSize() -> u32;
        pub fn ethereum_externalCodeCopy(
            addressOffset: *const u32,
            resultOffset: *const u32,
            codeOffset: u32,
            length: u32,
        );
        pub fn ethereum_getExternalCodeSize(addressOfset: *const u32) -> u32;
        pub fn ethereum_storageLoad(keyOffset: *const u32, resultOffset: *const u32);
        pub fn ethereum_storageStore(keyOffset: *const u32, valueOffset: *const u32);
        pub fn ethereum_selfDestruct(addressOffset: *const u32) -> !;
    }
}

fn unsafe_alloc_buffer(len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        ret.set_len(len);
    }
    ret
}

#[derive(Default, Copy, Clone)]
pub struct Uint128 {
    pub bytes: [u8; 16],
}

#[derive(Default, Copy, Clone)]
pub struct Bytes20 {
    pub bytes: [u8; 20],
}

#[derive(Default, Copy, Clone)]
pub struct Bytes32 {
    pub bytes: [u8; 32],
}

type EtherValue = Uint128;
type Address = Bytes20;
type StorageKey = Bytes32;
type StorageValue = Bytes32;
type Topic = Bytes32;
type Hash = Bytes32;
// TODO: is this a number of a bytestring?
type Difficulty = Bytes32;

pub enum Error {
    OutOfBoundsCopy,
}

pub enum CallResult {
    Successful,
    Failure,
    Revert,
}

pub enum CreateResult {
    Successful(Address),
    Failure,
    Revert,
}

pub fn consume_gas(amount: u64) {
    unsafe {
        native::ethereum_useGas(amount);
    }
}

pub fn gas_left() -> u64 {
    unsafe { native::ethereum_getGasLeft() }
}

pub fn current_address() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getAddress(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

pub fn external_balance(address: &Address) -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getBalance(
            address.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        );
    }

    ret
}

pub fn block_coinbase() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getBlockCoinbase(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

pub fn block_difficulty() -> Difficulty {
    let mut ret = Difficulty::default();

    unsafe {
        native::ethereum_getBlockDifficulty(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

pub fn block_gas_limit() -> u64 {
    unsafe { native::ethereum_getBlockGasLimit() }
}

pub fn block_hash(number: u64) -> Hash {
    let mut ret = Hash::default();

    unsafe {
        native::ethereum_getBlockHash(number, ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

pub fn block_number() -> u64 {
    unsafe { native::ethereum_getBlockNumber() }
}

pub fn block_timestamp() -> u64 {
    unsafe { native::ethereum_getBlockTimestamp() }
}

pub fn tx_gas_price() -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getTxGasPrice(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

pub fn tx_origin() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getTxOrigin(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

fn log(
    data: &[u8],
    topic_count: usize,
    topic1: *const u8,
    topic2: *const u8,
    topic3: *const u8,
    topic4: *const u8,
) {
    unsafe {
        native::ethereum_log(
            data.as_ptr() as *const u32,
            data.len() as u32,
            topic_count as u32,
            topic1 as *const u32,
            topic2 as *const u32,
            topic3 as *const u32,
            topic4 as *const u32,
        );
    }
}

pub fn log0(data: &[u8]) {
    log(
        data,
        0,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

pub fn log1(data: &[u8], topic1: &Topic) {
    log(
        data,
        1,
        topic1.bytes.as_ptr() as *const u8,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

pub fn log2(data: &[u8], topic1: &Topic, topic2: &Topic) {
    log(
        data,
        2,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

pub fn log3(data: &[u8], topic1: &Topic, topic2: &Topic, topic3: &Topic) {
    log(
        data,
        3,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        topic3.bytes.as_ptr() as *const u8,
        0 as *const u8,
    )
}

pub fn log4(data: &[u8], topic1: &Topic, topic2: &Topic, topic3: &Topic, topic4: &Topic) {
    log(
        data,
        4,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        topic3.bytes.as_ptr() as *const u8,
        topic4.bytes.as_ptr() as *const u8,
    )
}

pub fn call_mutable(
    gas_limit: u64,
    address: &Address,
    value: &EtherValue,
    data: &[u8],
) -> CallResult {
    let ret = unsafe {
        native::ethereum_call(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

pub fn call_code(gas_limit: u64, address: &Address, value: &EtherValue, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callCode(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

pub fn call_delegate(gas_limit: u64, address: &Address, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callDelegate(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

pub fn call_static(gas_limit: u64, address: &Address, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callStatic(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

pub fn create(value: &EtherValue, data: &[u8]) -> CreateResult {
    let mut result: Address = Address::default();

    let ret = unsafe {
        native::ethereum_create(
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
            result.bytes.as_mut_ptr() as *const u32,
        )
    };

    match ret {
        0 => CreateResult::Successful(result),
        1 => CreateResult::Failure,
        2 => CreateResult::Revert,
        _ => panic!(),
    }
}

pub fn unsafe_calldata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_callDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

pub fn calldata_acquire() -> Vec<u8> {
    unsafe_calldata_copy(0, calldata_size())
}

pub fn calldata_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = calldata_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_calldata_copy(from, length))
    }
}

pub fn calldata_size() -> usize {
    unsafe { native::ethereum_getCallDataSize() as usize }
}

pub fn caller() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getCaller(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

pub fn callvalue() -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getCallValue(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

pub fn unsafe_code_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_codeCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

pub fn code_acquire() -> Vec<u8> {
    unsafe_code_copy(0, code_size())
}

pub fn code_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = code_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_code_copy(from, length))
    }
}

pub fn code_size() -> usize {
    unsafe { native::ethereum_getCodeSize() as usize }
}

pub fn unsafe_external_code_copy(address: &Address, from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_externalCodeCopy(
            address.bytes.as_ptr() as *const u32,
            ret.as_mut_ptr() as *const u32,
            from as u32,
            length as u32,
        );
    }

    ret
}

pub fn external_code_acquire(address: &Address) -> Vec<u8> {
    unsafe_external_code_copy(address, 0, external_code_size(address))
}

pub fn external_code_copy(address: &Address, from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = external_code_size(address);

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_external_code_copy(address, from, length))
    }
}

pub fn external_code_size(address: &Address) -> usize {
    unsafe { native::ethereum_getExternalCodeSize(address.bytes.as_ptr() as *const u32) as usize }
}

pub fn unsafe_returndata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_returnDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

pub fn returndata_acquire() -> Vec<u8> {
    unsafe_returndata_copy(0, returndata_size())
}

pub fn returndata_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = returndata_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_returndata_copy(from, length))
    }
}

pub fn returndata_size() -> usize {
    unsafe { native::ethereum_getReturnDataSize() as usize }
}

pub fn revert() -> ! {
    unsafe {
        native::ethereum_revert(0 as *const u32, 0 as u32);
    }
}

pub fn revert_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_revert(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn finish() -> ! {
    unsafe {
        native::ethereum_finish(0 as *const u32, 0 as u32);
    }
}

pub fn finish_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_finish(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn storage_load(key: &StorageKey) -> StorageValue {
    let mut ret = StorageValue::default();

    unsafe {
        native::ethereum_storageLoad(
            key.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        );
    }

    ret
}

pub fn storage_store(key: &StorageKey, value: &StorageValue) {
    unsafe {
        native::ethereum_storageStore(
            key.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
        );
    }
}

pub fn selfdestruct(address: &Address) -> ! {
    unsafe {
        native::ethereum_selfDestruct(address.bytes.as_ptr() as *const u32);
    }
}
