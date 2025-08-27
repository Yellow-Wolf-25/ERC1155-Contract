
pragma solidity 0.8.30;

contract IterableMapping {
    mapping(address => uint) public balances;
    mapping(address => bool) public inserted;
    address[] public keys;

    function insert(address _key, uint _value) public {
        require(!inserted[_key], "Key already exists");
        balances[_key] = _value;
        keys.push(_key);
        inserted[_key] = true;
    }

    function getKeys() public view returns (address[] memory) {
        return keys;
    }

    function getSize() public view returns (uint) {
        return keys.length;
    }

    function getBalance(address _key) public view returns (uint) {
        require(inserted[_key], "Key does not exist");
        return balances[_key];
    }

    function getFirstKey() public view returns (uint256) {
        require(keys.length > 0, "No keys exist");
        return balances[keys[0]];
    }

    function getLastKey() public view returns (uint256) {
        require(keys.length > 0, "No keys exist");
        return balances[keys[keys.length - 1]];
    }
}