require ('dotenv').config();
const {ethers} = require("ethers");

const provider = new ethers.providers.WebSocketProvider(process.env.#ENV_LABEL#)

const inspectBlock = async () => {
	const blockNumber = await proivder.getBlockNumber()
	console.log(`Block# ${blockNumber}\n`)
    //Can be modified to track transactions of a given address or do other kind of forensics on the chain
	const blockInfo = await proivder.getBlock(blockNumber)
	console.log(blockInfo)
}

inspectBlock()